use std::collections::HashMap;

#[aoc_generator(day6)]
fn parse(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(')');
            (
                parts.next().expect("no anchor").to_string(),
                parts.next().expect("no orbiter").to_string(),
            )
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(orbits: &[(String, String)]) -> usize {
    let orbits = get_orbits(orbits);
    let mut orbit_cache: HashMap<String, Vec<String>> = HashMap::new();
    orbits
        .keys()
        .for_each(|object| compute_all_orbits(&orbits, &object, &mut orbit_cache));
    orbits
        .keys()
        .map(|object| orbit_cache.get(object).expect("not in cache").len())
        .sum()
}

fn get_orbits(orbits: &[(String, String)]) -> HashMap<String, String> {
    orbits
        .iter()
        .cloned()
        .map(|(anchor, orbiter)| (orbiter, anchor))
        .collect()
}

fn compute_all_orbits(
    orbits: &HashMap<String, String>,
    object: &str,
    orbit_cache: &mut HashMap<String, Vec<String>>,
) {
    if orbit_cache.contains_key(object) {
        return;
    }

    let anchor = orbits.get(object);

    if let Some(anchor) = anchor {
        compute_all_orbits(orbits, anchor, orbit_cache);
        let mut all_orbits = orbit_cache.get(anchor).expect("not computed").clone();
        all_orbits.push(anchor.clone());
        orbit_cache.insert(object.to_string(), all_orbits);
    } else {
        orbit_cache.insert(object.to_string(), vec![]);
    }
}

#[aoc(day6, part2)]
fn part2(orbits: &[(String, String)]) -> usize {
    let orbits = get_orbits(orbits);
    let mut orbit_cache: HashMap<String, Vec<String>> = HashMap::new();
    compute_all_orbits(&orbits, "YOU", &mut orbit_cache);
    compute_all_orbits(&orbits, "SAN", &mut orbit_cache);
    let my_path = orbit_cache.remove("YOU").unwrap();
    let san_path = orbit_cache.remove("SAN").unwrap();
    let mut my_iter = my_path.into_iter();
    let mut san_iter = san_path.into_iter();
    while my_iter.next() == san_iter.next() {}
    my_iter.count() + san_iter.count()
}
