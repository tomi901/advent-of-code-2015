use xmas::direction::DIRECTIONS_8;
use xmas::map2d::ByteMap;

pub const ON: u8 = b'#';
pub const OFF: u8 = b'.';

pub fn simulate_step(state: &ByteMap) -> ByteMap {
    let mut new_state = state.clone();

    for (pos, &current) in state.iter_with_points() {
        let alive_count = DIRECTIONS_8
            .iter()
            .flat_map(|&d| state.get_tile(pos + d))
            .filter(|&other| other == &ON)
            .count();

        match (current, alive_count) {
            (ON, 2 | 3) => {},
            (ON, _) => {
                new_state.set_tile(pos, OFF);
            },
            (OFF, 3) => {
                new_state.set_tile(pos, ON);
            },
            (OFF, _) => {},
            _ => unreachable!()
        }
    }

    new_state
}

pub fn simulate_step_corners_on(state: &ByteMap) -> ByteMap {
    let mut new_state = state.clone();
    let max_x = state.width() as isize - 1;
    let max_y = state.height() as isize - 1;

    for (pos, &current) in state.iter_with_points() {
        let is_corner = (pos.0 == 0 || pos.0 == max_x) && (pos.1 == 0 || pos.1 == max_y);
        if is_corner {
            continue;
        }

        let alive_count = DIRECTIONS_8
            .iter()
            .flat_map(|&d| state.get_tile(pos + d))
            .filter(|&other| other == &ON)
            .count();

        match (current, alive_count) {
            (ON, 2 | 3) => {},
            (ON, _) => {
                new_state.set_tile(pos, OFF);
            },
            (OFF, 3) => {
                new_state.set_tile(pos, ON);
            },
            (OFF, _) => {},
            _ => unreachable!()
        }
    }

    new_state
}

pub fn alive_count(state: &ByteMap) -> usize {
    state.iter().filter(|&t| t == &ON).count()
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use super::*;
    
    #[test]
    pub fn part_2_test_case() {
        let input = r"##.#.#
...##.
#....#
..#...
#.#..#
####.#";
        let mut state = ByteMap::from_str(input).unwrap();
        for _ in 0..5 {
            state = simulate_step_corners_on(&state);
        }
        assert_eq!(alive_count(&state), 17);
    }
}
