use std::str::FromStr;
use anyhow::anyhow;

pub struct GiftBox {
    length: u64,
    width: u64,
    height: u64,
}

impl FromStr for GiftBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [length, width, height]: [u64; 3] = s
            .split('x')
            .map(|d| d.parse())
            .collect::<Result<Vec<_>, _>>()?
            .try_into()
            .map_err(|v: Vec<_>| anyhow!("Expected 3 dimensions, found {}", v.len()))?;

        Ok(Self {
            length,
            width,
            height,
        })
    }
}

impl GiftBox {
    fn volume(&self) -> u64 {
        self.length * self.width * self.height
    }
    
    fn perimeters(&self) -> [u64; 3] {
        [
            self.length * 2 + self.width * 2,
            self.width * 2 + self.height * 2,
            self.length * 2 + self.height * 2,
        ]
    }

    fn surface_faces(&self) -> [u64; 3] {
        [self.length * self.width, self.width * self.height, self.length * self.height]
    }

    pub fn needed_wrap_surface_area(&self) -> u64 {
        let faces = self.surface_faces();
        let smallest = faces.iter().min().unwrap();
        faces.iter().sum::<u64>() * 2 + smallest
    }
    
    pub fn needed_ribbon_length(&self) -> u64 {
        let perimeters = self.perimeters();
        let smallest = perimeters.iter().min().unwrap();
        smallest + self.volume()
    }
}
