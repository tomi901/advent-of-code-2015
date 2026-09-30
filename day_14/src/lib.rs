use std::cmp::min;
use std::ops::Div;
use std::str::FromStr;

pub struct Reindeer {
    name: String,
    speed: i64,
    movement_time: i64,
    rest_time: i64,
}

impl Reindeer {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn distance_after(&self, time: i64) -> i64 {
        let section_time = self.movement_time + self.rest_time;
        let complete_section_count = time.div(section_time);

        let complete_sections_time = complete_section_count * section_time;
        let complete_sections_distance = self.speed * self.movement_time * complete_section_count;

        // And then we calculate the "incomplete section" if still remaining
        let remaining_time = time - complete_sections_time;
        let remaining_distance = self.speed * min(remaining_time, self.movement_time);

        complete_sections_distance + remaining_distance
    }
}

impl FromStr for Reindeer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_whitespace().collect::<Vec<_>>();
        Ok(Reindeer {
            name: split[0].to_string(),
            speed: split[3].parse()?,
            movement_time: split[6].parse()?,
            rest_time: split[13].parse()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn example_comet() {
        let reindeer = Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            movement_time: 10,
            rest_time: 127,
        };
        assert_eq!(reindeer.distance_after(1000), 1120);
    }

    #[test]
    pub fn example_dancer() {
        let reindeer = Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            movement_time: 11,
            rest_time: 162,
        };
        assert_eq!(reindeer.distance_after(1000), 1056);
    }
}
