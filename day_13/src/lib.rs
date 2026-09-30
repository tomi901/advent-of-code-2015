use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::anyhow;

pub type Happiness = i64;

#[derive(Debug, Default)]
pub struct GuestList {
    guests: Vec<String>,
    happiness: HashMap<(usize, usize), Happiness>,
}

impl GuestList {
    pub fn get_name(&self, index: usize) -> &str {
        &self.guests[index]
    }
    
    fn get_guest_index(&self, name: &str) -> Option<usize> {
        self.guests
            .iter()
            .position(|n| n == name)
    }

    fn get_or_create_guest(&mut self, name: &str) -> usize {
        match self.get_guest_index(name) {
            Some(i) => i,
            None => {
                let new_index = self.guests.len();
                self.guests.push(name.to_string());
                new_index
            }
        }
    }

    fn evaluate_happiness(&self, guest_list: &[usize]) -> Happiness {
        (0..guest_list.len())
            .map(|i| {
                let current = guest_list[i];
                let next = guest_list[(i + 1) % guest_list.len()];
                let previous = guest_list[(i + guest_list.len() - 1) % guest_list.len()];
                self.get_happiness_next_to(current, previous) + self.get_happiness_next_to(current, next)
            })
            .sum()
    }
    
    fn get_happiness_next_to(&self, from: usize, to: usize) -> Happiness {
        self.happiness[&(from, to)]
    }

    fn get_happiness_for_guest(&self, index: usize) -> (Happiness, Happiness) {
        let next = (index + 1) % self.guests.len();
        let previous = (index + self.guests.len() - 1) % self.guests.len();
        (self.happiness[&(index, previous)], self.happiness[&(index, next)])
    }

    fn get_happiness_for_guest_name(&self, name: &str) -> (Happiness, Happiness) {
        let index = self.get_guest_index(name).unwrap();
        self.get_happiness_for_guest(index)
    }
}

impl FromStr for GuestList {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut list = GuestList::default();
        for line in s.lines() {
            let split: Vec<_> = line.split_whitespace().collect();
            let name_a = split[0];
            let name_b = split[10].trim_end_matches('.');

            let unsigned_delta = split[3].parse::<i64>()?;
            let delta = match split[2] {
                "gain" => unsigned_delta,
                "lose" => -unsigned_delta,
                word => return Err(anyhow!("Unknown case: {}", word)),
            };

            let guest_a = list.get_or_create_guest(name_a);
            let guest_b = list.get_or_create_guest(name_b);

            list.happiness.insert((guest_a, guest_b), delta);
        }
        Ok(list)
    }
}

pub fn find_most_optimal_happiness(guests: &GuestList) -> Option<(Happiness, Vec<usize>)> {
    let mut candidate_list = Vec::new();
    find_most_optimal_happiness_internal(guests, &mut candidate_list)
}

fn find_most_optimal_happiness_internal(
    guests: &GuestList,
    candidate_list: &mut Vec<usize>,
) -> Option<(Happiness, Vec<usize>)> {
    if candidate_list.len() == guests.guests.len() {
        let list_copy = candidate_list.clone();
        return Some((guests.evaluate_happiness(&candidate_list[..]), list_copy));
    }

    let already_present = candidate_list.iter().cloned().collect::<HashSet<_>>();
    let mut result: Option<(Happiness, Vec<usize>)> = None;
    for i in (0..guests.guests.len()).filter(|i| !already_present.contains(i)) {
        candidate_list.push(i);
        if let Some(found) = find_most_optimal_happiness_internal(guests, candidate_list) {
            if result.as_ref().is_none_or(|&(r, _)| found.0 > r) {
                // println!("Found: {}", found.0);
                result = Some(found);
            }
        }
        candidate_list.pop();
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &'static str = r"Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    fn sample_list() -> GuestList {
        SAMPLE.parse().unwrap()
    }

    #[test]
    pub fn sample_list_alice() {
        let list = sample_list();
        let delta = list.get_happiness_for_guest_name("Alice");
        assert_eq!(delta, (-2, 54));
    }

    #[test]
    pub fn part_1_sample() {
        let list = sample_list();
        let delta = find_most_optimal_happiness(&list)
            .unwrap()
            .0;
        assert_eq!(delta, 330);
    }
}
