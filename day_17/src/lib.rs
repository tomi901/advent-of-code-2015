use std::cmp::Reverse;
use std::collections::HashSet;

pub fn calculate_combinations(capacity: u32, containers: &[u32]) -> usize {
    let mut used_indices = HashSet::new();
    let mut sorted = containers.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(|&x| Reverse(x));
    calculate_combinations_internal(capacity, &sorted, &mut used_indices, None, None).0
}

pub fn calculate_combinations_minimum_containers(capacity: u32, containers: &[u32]) -> usize {
    let mut used_indices = HashSet::new();
    let mut sorted = containers.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(|&x| Reverse(x));

    let (_, Some(minimum_amount)) = calculate_combinations_internal(
        capacity,
        &sorted,
        &mut used_indices,
        None,
        None,
    ) else {
        return 0;
    };

    // dbg!(minimum_amount);
    calculate_combinations_internal(
        capacity,
        &sorted,
        &mut used_indices,
        None,
        Some(minimum_amount),
    ).0
}

/// (combinations, minimum amount of containers combination)
fn calculate_combinations_internal(
    capacity: u32,
    containers_sorted: &[u32],
    used: &mut HashSet<usize>,
    last_index: Option<usize>,
    use_limit: Option<usize>,
) -> (usize, Option<usize>) {
    if capacity == 0 {
        return (1, Some(used.len()));
    }

    if use_limit.is_some_and(|limit| used.len() >= limit) {
        // println!("Limit ({})", use_limit.unwrap());
        return (0, None);
    }

    let skip = last_index.unwrap_or(0);

    let mut sum = 0;
    let mut minimum_size_combination = None;
    for (i, &size) in containers_sorted.iter().enumerate().skip(skip) {
        if size > capacity || used.contains(&i) {
            continue;
        }

        used.insert(i);
        let (combinations_found, minimum_found) = calculate_combinations_internal(
            capacity - size,
            containers_sorted,
            used,
            Some(i),
            use_limit,
        );
        used.remove(&i);

        sum += combinations_found;
        match (minimum_size_combination, minimum_found) {
            (Some(current), Some(found)) if found < current => {
                minimum_size_combination = minimum_found;
            },
            (None, Some(_)) => minimum_size_combination = minimum_found,
            _ => {},
        }
    }
    (sum, minimum_size_combination)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAPACITY: u32 = 25;
    const CONTAINERS: &[u32] = &[20, 15, 10, 5, 5];

    #[test]
    pub fn part_1_test_case() {
        assert_eq!(
            calculate_combinations(CAPACITY, CONTAINERS),
            4,
        );
    }

    #[test]
    pub fn part_2_test_case() {
        assert_eq!(
            calculate_combinations_minimum_containers(CAPACITY, CONTAINERS),
            3,
        );
    }
}
