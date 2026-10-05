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
            .all(|(prop, value)| hints.get(prop).is_some_and(|x| value == x))
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
