use std::str::FromStr;
use anyhow::{anyhow, Context};

pub struct GiftBox {
    length: u64,
    width: u64,
    height: u64,
}

impl FromStr for GiftBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('x');
        
        let length = split.next().context("length missing")?.parse()?;
        let width = split.next().context("width missing")?.parse()?;
        let height = split.next().context("height missing")?.parse()?;
        
        if split.next().is_some() {
            return Err(anyhow!("too many dimensions, 3 expected"));
        }
        
        Ok(Self {
            length,
            width,
            height,
        })
    }
}

impl GiftBox {
    fn surface_faces(&self) -> [u64; 3] {
        [self.length * self.width, self.width * self.height, self.length * self.height]
    }
    
    pub fn surface_area_needed(&self) -> u64 {
        let faces = self.surface_faces();
        let smallest = faces.iter().min().unwrap();
        faces.iter().sum::<u64>() * 2 + smallest
    }
}
