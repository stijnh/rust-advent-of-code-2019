use crate::common::*;
use std::collections::HashMap;

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut orbits = HashMap::new();
    let mut neighbors = HashMap::new();

    for line in read_input("day06")? {
        if let Some(index) = line.find(')') {
            let src = line[..index].to_string();
            let dst = line[(index + 1)..].to_string();

            orbits.insert(dst.clone(), src.clone());

            for &(a, b) in &[(&src, &dst), (&dst, &src)] {
                neighbors.entry(a.clone()).or_insert(vec![]).push(b.clone());
            }
        }
    }

    let mut total_orbits = 0;
    for origin in orbits.keys() {
        let mut current = origin;

        // for each planet, traverse back to the root
        while let Some(t) = orbits.get(current) {
            total_orbits += 1;
            current = t;
        }
    }

    println!("answer A: {}", total_orbits);

    // basic depth-first-search implementation
    let mut queue = vec![("YOU", 0)];
    let mut visited: HashMap<&str, u32> = HashMap::new();

    while let Some((current, dist)) = queue.pop() {
        for neighbor in &neighbors[current] {
            if !visited.contains_key(&**neighbor) {
                visited.insert(neighbor, dist + 1);
                queue.push((neighbor, dist + 1));
            }
        }
    }

    // -2 since YOU and SAN are not planets some we do'nt need the
    // first and final transfer
    println!("answer B: {}", visited["SAN"] - 2);

    Ok(())
}
