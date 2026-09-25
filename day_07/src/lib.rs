use std::collections::HashMap;
use std::str::FromStr;
use anyhow::{anyhow, Context};
use self::Expression::*;

pub type CircuitCache = HashMap<String, u16>;

#[derive(Debug, Clone)]
pub struct Circuit {
    statements: HashMap<String, Expression>,
}

impl Circuit {
    pub fn set(&mut self, identifier: String, expression: Expression) {
        self.statements.insert(identifier, expression);
    }
}

impl Circuit {
    pub fn evaluate(&self, identifier: &str, cache: &mut CircuitCache) -> anyhow::Result<u16> {
        if let Some(&cached) = cache.get(identifier) {
            return Ok(cached);
        }
        
        let statement = self
            .statements
            .get(identifier)
            .with_context(|| format!("Identifier \"{}\" not found.", identifier))?;

        let result = match statement {
            And(a, b) => self.eval(a, cache)? & self.eval(b, cache)?,
            Or(a, b) => self.eval(a, cache)? | self.eval(b, cache)?,
            LShift(a, b) => self.eval(a, cache)? << self.eval(b, cache)?,
            RShift(a, b) => self.eval(a, cache)? >> self.eval(b, cache)?,
            Not(x) => !self.eval(x, cache)?,
            Val(x) => self.eval(x, cache)?,
        };
        
        cache.insert(identifier.to_string(), result);
        Ok(result)
    }

    fn eval(&self, value: &Value, cache: &mut CircuitCache) -> anyhow::Result<u16> {
        match value {
            Value::Constant(n) => Ok(*n),
            Value::Ref(identifier) => self.evaluate(identifier, cache),
        }
    }
}

impl FromStr for Circuit {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let statements = s.lines()
            .map(|l| {
                let (expr, identifier) = l
                    .split_once(" -> ")
                    .with_context(|| format!("Invalid statement: {}", l))?;
                Expression::from_str(expr)
                    .map(|e| (identifier.to_string(), e))
            })
            .collect::<Result<HashMap<_, _>, _>>()?;
        Ok(Self { statements })
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Val(Value),
    And(Value, Value),
    Or(Value, Value),
    LShift(Value, Value),
    RShift(Value, Value),
    Not(Value),
}

impl Expression {
    pub fn constant(value: u16) -> Self {
        Val(Value::Constant(value))
    }
}

impl FromStr for Expression {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segments: Vec<_> = s.split_whitespace().collect();
        let statement = match segments.len() {
            1 => Val(Value::from_str(s)?),
            2 => match segments[0] {
                "NOT" => Not(Value::from_str(segments[1])?),
                _ => return Err(anyhow!("Invalid unary operator: {}", segments[0])),
            },
            3 => match segments[1] {
                "AND" => And(Value::from_str(segments[0])?, Value::from_str(segments[2])?),
                "OR" => Or(Value::from_str(segments[0])?, Value::from_str(segments[2])?),
                "LSHIFT" => LShift(Value::from_str(segments[0])?, Value::from_str(segments[2])?),
                "RSHIFT" => RShift(Value::from_str(segments[0])?, Value::from_str(segments[2])?),
                _ => return Err(anyhow!("Invalid unary operator: {}", segments[0])),
            },
            _ => return Err(anyhow!("Invalid statement length ({})", segments.len())),
        };
        Ok(statement)
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Constant(u16),
    Ref(String),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<u16>() {
            Ok(n) => Value::Constant(n),
            Err(_) => Value::Ref(s.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn part_1_example() {
        let input = r"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let circuit = Circuit::from_str(input).unwrap();
        let mut cache = CircuitCache::default();

        assert_eq!(circuit.evaluate("x", &mut cache).unwrap(), 123);
        assert_eq!(circuit.evaluate("y", &mut cache).unwrap(), 456);
        assert_eq!(circuit.evaluate("d", &mut cache).unwrap(), 72);
        assert_eq!(circuit.evaluate("e", &mut cache).unwrap(), 507);
        assert_eq!(circuit.evaluate("f", &mut cache).unwrap(), 492);
        assert_eq!(circuit.evaluate("g", &mut cache).unwrap(), 114);
        assert_eq!(circuit.evaluate("h", &mut cache).unwrap(), 65412);
        assert_eq!(circuit.evaluate("i", &mut cache).unwrap(), 65079);
    }
}
