use crate::common::*;
use std::cmp::min;

fn presum(signal: &[i8]) -> Vec<i64> {
    let mut output = vec![0; signal.len() + 1];
    let mut sum = 0;

    for (&x, y) in zip(signal, &mut output[1..]) {
        sum += x as i64;
        *y = sum;
    }

    output
}

#[inline(never)]
fn apply_pattern(signal: &[i8], presum: &[i64], repeats: usize) -> i8 {
    let num = signal.len();
    let mut sum = 0i64;
    let mut index = repeats - 1;

    while index < num {
        let a = index;
        let b = min(a + repeats, num);
        let c = min(b + repeats, num);
        let d = min(c + repeats, num);
        index = min(d + repeats, num);

        sum += (presum[b] - presum[a]) - (presum[d] - presum[c]);
    }

    (sum.abs() % 10) as i8
}

fn fft(signal: &[i8], times: usize, offset: usize) -> Vec<i8> {
    let n = signal.len();
    let mut signal = signal.to_vec();
    let mut output = signal.to_vec();

    for t in 0..times {
        println!("timestep {:?}", t);
        let presum = presum(&signal);

        for i in offset..n {
            output[i] = apply_pattern(&signal, &presum, i + 1);
        }

        signal.clone_from(&output);
    }

    signal
}

fn array2index(arr: &[i8]) -> usize {
    let mut output = 0;

    for &x in arr {
        output = (output * 10) + (x as usize);
    }

    output
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let signal = read_input("day16")?[0]
        .chars()
        .map(|c| (c as i8) - ('0' as i8))
        .collect::<Vec<_>>();

    let result = fft(&signal, 100, 0);
    println!("answer A: {:?}", &result[..8]);

    let mut repeated_signal = vec![];
    for _ in 0..10_000 {
        repeated_signal.extend(&signal);
    }

    let index = array2index(&signal[..7]);
    let result = fft(&repeated_signal, 100, index);

    println!("answer B: {:?}", &result[index..index + 8]);

    Ok(())
}
