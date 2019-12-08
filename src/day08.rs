use crate::common::*;

fn count(slice: &[char], c: char) -> usize {
    slice.iter().map(|&a| (a == c) as usize).sum()
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let (width, height) = (25, 6);
    let buffer = read_input("day08")?[0].trim().chars().collect::<Vec<_>>();

    let layers = buffer.chunks_exact(width * height).collect::<Vec<_>>();

    let fewest_zeros = layers.iter().min_by_key(|img| count(img, '0')).unwrap();

    let answer = count(fewest_zeros, '1') * count(fewest_zeros, '2');
    println!("answer A: {}", answer);

    let mut img = vec!['2'; width * height];
    for layer in layers {
        for (a, b) in zip(&mut img, layer) {
            if *a == '2' {
                *a = *b;
            }
        }
    }

    println!("answer B:");

    for row in img.chunks(width) {
        for &c in row {
            print!("{}", iff!(c == '1', '*', ' '));
        }
        println!();
    }

    Ok(())
}
