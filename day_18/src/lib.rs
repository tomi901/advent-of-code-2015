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

pub fn alive_count(state: ByteMap) -> usize {
    state.iter().filter(|&t| t == &ON).count()
}
