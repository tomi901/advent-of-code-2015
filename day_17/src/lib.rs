use std::cmp::Reverse;
use std::collections::HashSet;

pub fn count_combinations(liters: u32, containers: &[u32]) -> usize {
    let mut used_indices = HashSet::new();
    let mut sorted = containers.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(|&x| Reverse(x));
    count_combinations_internal(liters, &sorted, &mut used_indices, None, None).0
}

pub fn count_minimum_containers_combinations(liters: u32, containers: &[u32]) -> usize {
    let mut used_indices = HashSet::new();
    let mut sorted = containers.iter().cloned().collect::<Vec<_>>();
    sorted.sort_by_key(|&x| Reverse(x));

    let (_, Some(minimum_amount)) = count_combinations_internal(
        liters,
        &sorted,
        &mut used_indices,
        None,
        None,
    ) else {
        return 0;
    };

    // dbg!(minimum_amount);
    count_combinations_internal(
        liters,
        &sorted,
        &mut used_indices,
        None,
        Some(minimum_amount),
    ).0
}

/// (combinations, minimum amount of containers combination)
fn count_combinations_internal(
    liters: u32,
    containers_sorted: &[u32],
    // Not really necessary, since we sort the containers to avoid duplicates
    used: &mut HashSet<usize>,
    last_index: Option<usize>,
    use_limit: Option<usize>,
) -> (usize, Option<usize>) {
    if liters == 0 {
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
        if size > liters || used.contains(&i) {
            continue;
        }

        used.insert(i);
        let (combinations_found, minimum_found) = count_combinations_internal(
            liters - size,
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
            count_combinations(CAPACITY, CONTAINERS),
            4,
        );
    }

    #[test]
    pub fn part_2_test_case() {
        assert_eq!(
            count_minimum_containers_combinations(CAPACITY, CONTAINERS),
            3,
        );
    }
}
