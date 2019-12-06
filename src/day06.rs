use crate::common::*;
use std::collections::HashMap;

pub(crate) fn run(_args: &[&str]) -> Result {
    // parse input
    let mut orbits = HashMap::new();

    for line in read_input("day06")? {
        if let Some(index) = line.find(')') {
            let src = line[..index].to_string();
            let dst = line[(index + 1)..].to_string();

            orbits.insert(dst.clone(), src.clone());
        }
    }

    // for each planet, traverse back to the root and count the number of hops
    let mut total_orbits = 0;
    for origin in orbits.keys() {
        let mut current = origin;

        while let Some(t) = orbits.get(current) {
            total_orbits += 1;
            current = t;
        }
    }

    println!("answer A: {}", total_orbits);

    // get both backwards and forward edges
    let mut neighbors = HashMap::<&str, Vec<&str>>::new();
    for (dst, src) in &orbits {
        neighbors.entry(src).or_default().push(dst);
        neighbors.entry(dst).or_default().push(src);
    }

    // basic depth-first-search implementation
    let mut queue = vec![("YOU", 0)];
    let mut visited = HashMap::<&str, u32>::new();

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
