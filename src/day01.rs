use crate::common::*;

/// Simply calculates the required fuel for the given mass.
fn fuel_for_mass_simple(x: f64) -> f64 {
    (x / 3.0).floor() - 2.0
}

/// Calculates the required fuel for the given mass, and fuel for this required fuel, and the
/// fuel for this required fuel, ad infinitum.
fn fuel_for_mass_complex(x: f64) -> f64 {
    let f = fuel_for_mass_simple(x);
    iff!(f > 0.0, f + fuel_for_mass_complex(f), 0.0)
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut part_a = 0.0;
    let mut part_b = 0.0;

    for (index, line) in enumerate(read_input("day01")?) {
        let mass = line
            .parse::<f64>()
            .with_context(|| format!("failed to parse mass on line {}", index + 1))?;
        part_a += fuel_for_mass_simple(mass);
        part_b += fuel_for_mass_complex(mass);
    }

    println!("answer A: {}", part_a);
    println!("answer B: {}", part_b);

    Ok(())
}
