use std::cmp::Reverse;
use std::collections::HashSet;

pub fn calculate_combinations(capacity: u32, containers: &[u32]) -> usize {
    let mut used_indices = HashSet::new();
    let mut sorted = containers.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(|&x| Reverse(x));
    calculate_combinations_internal(capacity, &sorted, &mut used_indices, None)
}

fn calculate_combinations_internal(
    capacity: u32,
    containers_sorted: &[u32],
    used: &mut HashSet<usize>,
    last_index: Option<usize>,
) -> usize {
    // dbg!(capacity);
    if capacity == 0 {
        return 1;
    }

    let skip = match last_index {
        Some(i) => i + 1,
        None => 0,
    };

    let mut sum = 0;
    for (i, &size) in containers_sorted.iter().enumerate().skip(skip) {
        if size > capacity || used.contains(&i) {
            continue;
        }

        used.insert(i);
        sum += calculate_combinations_internal(
            capacity - size,
            containers_sorted,
            used,
            Some(i),
        );
        used.remove(&i);
    }
    sum
}
