use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use anyhow::{anyhow, Context};
use xmas::point2d::Point2D;
use crate::InstructionKind::*;

const TOGGLE_INS: &'static str = "toggle ";
const TURN_ON_INS: &'static str = "turn on ";
const TURN_OFF_INS: &'static str = "turn off ";

pub enum InstructionKind {
    Toggle,
    TurnOn,
    TurnOff,
}

pub struct Instruction {
    pub kind: InstructionKind,
    pub point_a: Point2D,
    pub point_b: Point2D,
}

impl Instruction {
    pub fn new(kind: InstructionKind, point_a: Point2D, point_b: Point2D) -> Self {
        Self {
            kind,
            point_a,
            point_b,
        }
    }

    pub fn range_count(&self) -> isize {
        let width = self.point_a.0.abs_diff(self.point_b.0) + 1;
        let height = self.point_a.1.abs_diff(self.point_b.1) + 1;
        (width * height) as isize
    }

    pub fn brightness_delta(&self) -> isize {
        match self.kind {
            Toggle => self.range_count() * 2,
            TurnOn => self.range_count(),
            TurnOff => -self.range_count(),
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, rest) = if s.starts_with(TOGGLE_INS) {
            (Toggle, s.trim_start_matches(TOGGLE_INS))
        } else if s.starts_with(TURN_ON_INS) {
            (TurnOn, s.trim_start_matches(TURN_ON_INS))
        } else if s.starts_with(TURN_OFF_INS) {
            (TurnOff, s.trim_start_matches(TURN_OFF_INS))
        } else {
            return Err(anyhow!("Unknown instruction: {}", s))
        };

        let (point_a, point_b) = rest
            .split_once(" through ")
            .with_context(|| format!("Invalid instruction split: {}", rest))?;

        let point_a = Point2D::from_str(point_a).context("Point A parsing")?;
        let point_b = Point2D::from_str(point_b).context("Point B parsing")?;

        Ok(Self {
            kind,
            point_a,
            point_b,
        })
    }
}

#[derive(Debug, Default)]
pub struct Lights {
    lit: HashSet<Point2D>,
}

impl Lights {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lit_count(&self) -> usize {
        self.lit.len()
    }

    pub fn apply_instruction(&mut self, s: &str) -> anyhow::Result<()> {
        let ins = Instruction::from_str(s)?;

        match ins.kind {
            Toggle => self.toggle(ins.point_a, ins.point_b),
            TurnOn => self.turn_on(ins.point_a, ins.point_b),
            TurnOff => self.turn_off(ins.point_a, ins.point_b),
        }
        Ok(())
    }

    pub fn turn_on(&mut self, from: Point2D, to: Point2D) {
        for p in from.iter_to_inclusive(to) {
            self.lit.insert(p);
        }
    }

    pub fn turn_off(&mut self, from: Point2D, to: Point2D) {
        for p in from.iter_to_inclusive(to) {
            self.lit.remove(&p);
        }
    }

    pub fn toggle(&mut self, from: Point2D, to: Point2D) {
        for p in from.iter_to_inclusive(to) {
            if self.lit.contains(&p) {
                self.lit.remove(&p);
            } else {
                self.lit.insert(p);
            }
        }
    }
}

#[derive(Debug, Default)]
pub struct LightsV2 {
    brightness: HashMap<Point2D, u32>,
}

impl LightsV2 {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn brightness(&self) -> u32 {
        self.brightness.values().sum()
    }
    
    pub fn apply_instruction(&mut self, instruction: Instruction) {
        let delta: i32 = match instruction.kind {
            Toggle => 2,
            TurnOn => 1,
            TurnOff => -1,
        };

        for p in instruction.point_a.iter_to_inclusive(instruction.point_b) {
            let state = self.brightness.get(&p).cloned().unwrap_or(0);
            if delta < 0 && (-delta) as u32 >= state {
                self.brightness.remove(&p);
                continue;
            }
            
            let new_state = (state as i32) + delta;
            self.brightness.insert(p, new_state as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_2_case_1() {
        let instruction = Instruction::new(TurnOn, Point2D(0, 0), Point2D(0, 0));
        assert_eq!(instruction.brightness_delta(), 1);
    }

    #[test]
    pub fn part_2_case_2() {
        let instruction = Instruction::new(Toggle, Point2D(0, 0), Point2D(999, 999));
        assert_eq!(instruction.brightness_delta(), 2_000_000);
    }
}
