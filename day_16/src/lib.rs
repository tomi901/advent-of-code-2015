use std::collections::HashMap;
use std::str::FromStr;
use anyhow::Context;

pub struct Sue {
    number: u32,
    properties: HashMap<String, u32>,
}

impl Sue {
    pub fn number(&self) -> u32 {
        self.number
    }

    pub fn is_suspect(&self, hints: &HashMap<String, u32>) -> bool {
        self.properties
            .iter()
            .all(|(prop, value)| hints.get(prop).is_some_and(|hint| value == hint))
    }

    pub fn is_suspect_v2(&self, hints: &HashMap<String, u32>) -> bool {
        self.properties
            .iter()
            .all(|(prop, &value)| hints
                .get(prop)
                .is_some_and(|&hint| is_v2_match(prop, value, hint)))
    }
}

fn is_v2_match(property: &str, value: u32, hint: u32) -> bool {
    match property {
        "cats" | "trees" => value > hint,
        "pomeranians" | "goldfish" => value < hint,
        _ => value == hint
    }
}

impl FromStr for Sue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, properties_str) = s.split_once(":").context("Invalid format")?;
        let number = name
            .split_whitespace()
            .skip(1)
            .next()
            .context("No number")?
            .parse()?;

        let properties = properties_str
            .split(",")
            .try_fold(HashMap::new(), |mut map, prop| -> anyhow::Result<_> {
                let (name, value) = prop
                    .split_once(":")
                    .context("Invalid format")?;
                let num_value: u32 = value.trim().parse()?;
                map.insert(name.trim().to_string(), num_value);
                Ok(map)
            })?;

        Ok(Self {
            number,
            properties,
        })
    }
}
