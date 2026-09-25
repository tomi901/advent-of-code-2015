use std::collections::HashSet;
use std::str::FromStr;
use anyhow::{anyhow, Context};
use xmas::point2d::Point2D;
use crate::Instruction::*;

const TOGGLE_INS: &'static str = "toggle ";
const TURN_ON_INS: &'static str = "turn on ";
const TURN_OFF_INS: &'static str = "turn off ";

enum Instruction {
    Toggle,
    TurnOn,
    TurnOff,
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
        let (instruction, rest) = if s.starts_with(TOGGLE_INS) {
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

        match instruction {
            Toggle => self.toggle(point_a, point_b),
            TurnOn => self.turn_on(point_a, point_b),
            TurnOff => self.turn_off(point_a, point_b),
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
