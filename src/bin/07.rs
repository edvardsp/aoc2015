use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(7);

#[derive(Clone, Copy)]
enum Var<'i> {
    Integer(u16),
    Name(&'i str),
}

impl<'i> From<&'i str> for Var<'i> {
    fn from(value: &'i str) -> Self {
        value
            .parse()
            .ok()
            .map(Var::Integer)
            .unwrap_or_else(|| Var::Name(value))
    }
}

enum Op<'i> {
    Assign {
        inp: Var<'i>,
        out: &'i str,
    },
    Not {
        inp: Var<'i>,
        out: &'i str,
    },
    And {
        lhs: Var<'i>,
        rhs: Var<'i>,
        out: &'i str,
    },
    Or {
        lhs: Var<'i>,
        rhs: Var<'i>,
        out: &'i str,
    },
    Lshift {
        lhs: Var<'i>,
        rhs: Var<'i>,
        out: &'i str,
    },
    Rshift {
        lhs: Var<'i>,
        rhs: Var<'i>,
        out: &'i str,
    },
}

impl<'i> Op<'i> {
    fn out(&self) -> &'i str {
        match self {
            Self::Assign { inp: _, out } => out,
            Self::Not { inp: _, out } => out,
            Self::And {
                lhs: _,
                rhs: _,
                out,
            } => out,
            Self::Or {
                lhs: _,
                rhs: _,
                out,
            } => out,
            Self::Lshift {
                lhs: _,
                rhs: _,
                out,
            } => out,
            Self::Rshift {
                lhs: _,
                rhs: _,
                out,
            } => out,
        }
    }
}

impl<'i> From<&'i str> for Op<'i> {
    fn from(value: &'i str) -> Self {
        let (op_str, out) = value.split_once(" -> ").unwrap();
        let tokens: Vec<_> = op_str.split_ascii_whitespace().collect();
        match *tokens.as_slice() {
            [inp] => Self::Assign {
                inp: inp.into(),
                out,
            },
            ["NOT", inp] => Self::Not {
                inp: inp.into(),
                out,
            },
            [lhs, "AND", rhs] => Self::And {
                lhs: lhs.into(),
                rhs: rhs.into(),
                out,
            },
            [lhs, "OR", rhs] => Self::Or {
                lhs: lhs.into(),
                rhs: rhs.into(),
                out,
            },
            [lhs, "LSHIFT", rhs] => Self::Lshift {
                lhs: lhs.into(),
                rhs: rhs.into(),
                out,
            },
            [lhs, "RSHIFT", rhs] => Self::Rshift {
                lhs: lhs.into(),
                rhs: rhs.into(),
                out,
            },
            _ => panic!("Invalid operation: {}", value),
        }
    }
}

#[derive(Default)]
struct Circuit<'i> {
    wires: HashMap<&'i str, u16>,
}

impl<'i> Circuit<'i> {
    fn new(ops: &[Op<'i>]) -> Self {
        let mut circuit = Circuit::default();
        circuit.resolve(ops);
        circuit
    }

    fn apply(&mut self, op: &Op<'i>) -> Option<()> {
        match op {
            Op::Assign { inp, out } => {
                let value = self.get(*inp)?;
                self.wires.insert(out, value);
            }
            Op::Not { inp, out } => {
                let value = !self.get(*inp)?;
                self.wires.insert(out, value);
            }
            Op::And { lhs, rhs, out } => {
                let lhs = self.get(*lhs)?;
                let rhs = self.get(*rhs)?;
                let value = lhs & rhs;
                self.wires.insert(out, value);
            }
            Op::Or { lhs, rhs, out } => {
                let lhs = self.get(*lhs)?;
                let rhs = self.get(*rhs)?;
                let value = lhs | rhs;
                self.wires.insert(out, value);
            }
            Op::Lshift { lhs, rhs, out } => {
                let lhs = self.get(*lhs)?;
                let rhs = self.get(*rhs)?;
                let value = lhs << rhs;
                self.wires.insert(out, value);
            }
            Op::Rshift { lhs, rhs, out } => {
                let lhs = self.get(*lhs)?;
                let rhs = self.get(*rhs)?;
                let value = lhs >> rhs;
                self.wires.insert(out, value);
            }
        }
        Some(())
    }

    fn resolve(&mut self, ops: &[Op<'i>]) {
        let mut queue = VecDeque::from_iter(ops);
        while let Some(op) = queue.pop_front() {
            if self.apply(op).is_none() {
                queue.push_back(op);
            }
        }
    }

    fn get<V>(&self, name: V) -> Option<u16>
    where
        V: Into<Var<'i>>,
    {
        match name.into() {
            Var::Integer(i) => Some(i),
            Var::Name(n) => self.wires.get(n).copied(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    let ops: Vec<_> = input.lines().map(Op::from).collect();
    Circuit::new(&ops).get("a")
}

pub fn part_two(input: &str) -> Option<u16> {
    let mut ops: Vec<_> = input.lines().map(Op::from).collect();
    let a = Circuit::new(&ops).get("a").unwrap();

    let b_out = ops.iter_mut().find(|op| op.out() == "b").unwrap();
    *b_out = Op::Assign {
        inp: Var::Integer(a),
        out: "b",
    };

    Circuit::new(&ops).get("a")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let ops: Vec<_> = input.lines().map(Op::from).collect();
        let circuit = Circuit::new(&ops);
        assert_eq!(circuit.get("d"), Some(72));
        assert_eq!(circuit.get("e"), Some(507));
        assert_eq!(circuit.get("f"), Some(492));
        assert_eq!(circuit.get("g"), Some(114));
        assert_eq!(circuit.get("h"), Some(65412));
        assert_eq!(circuit.get("i"), Some(65079));
        assert_eq!(circuit.get("x"), Some(123));
        assert_eq!(circuit.get("y"), Some(456));
    }
}
