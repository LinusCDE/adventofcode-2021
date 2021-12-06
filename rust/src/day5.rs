use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Display;
use std::{result::Result as StdResult, str::FromStr};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i16,
    y: i16,
}

impl Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl std::ops::Add for Pos {
    type Output = Pos;
    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Pos {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Pos {
    type Output = Pos;
    fn sub(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Pos {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

struct VentLine {
    from: Pos,
    to: Pos,
}

impl Display for VentLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}

impl FromStr for VentLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        ensure!(s.contains(" -> "));
        let numbers = s
            .replace(" -> ", ",")
            .split(",")
            .map(|n| n.parse())
            .collect::<StdResult<Vec<i16>, _>>()?;
        ensure!(numbers.len() == 4, "Invalid format!");
        VentLine::new(
            Pos {
                x: numbers[0],
                y: numbers[1],
            },
            Pos {
                x: numbers[2],
                y: numbers[3],
            },
        )
    }
}

impl VentLine {
    fn new(from: Pos, to: Pos) -> Result<Self> {
        let vent_line = VentLine { from, to };
        ensure!(
            vent_line.is_ortogonal() || vent_line.is_diagnonal(),
            "VentLine is not ortogonal nor diagonal"
        );
        Ok(vent_line)
    }

    fn iter_positions(&self) -> Result<VentLineIterator> {
        VentLineIterator::new(self)
    }

    /// Check if other point is either 0°, 90°, 180° or 270° from the given point
    fn is_ortogonal(&self) -> bool {
        self.from.x == self.to.x || self.from.y == self.to.y
    }

    /// Check if other point is either 45°, 135°, 225° or 315° from the given point
    fn is_diagnonal(&self) -> bool {
        let vec = self.from - self.to;
        !self.is_ortogonal() && vec.x.abs() == vec.y.abs()
    }
}

struct VentLineIterator {
    last: Pos,
    /// Only -1, 0 or 1 for points expected!
    vec: Pos,
    max: Pos,
}

impl VentLineIterator {
    fn new(vent_line: &VentLine) -> Result<Self> {
        // Should not result in missing == max since only 45°/90° angles are valid
        let vec = Pos {
            x: (vent_line.to.x - vent_line.from.x).clamp(-1, 1),
            y: (vent_line.to.y - vent_line.from.y).clamp(-1, 1),
        };
        Ok(Self {
            last: vent_line.from - vec,
            max: vent_line.to,
            vec,
        })
    }
}

impl Iterator for VentLineIterator {
    type Item = Pos;
    fn next(&mut self) -> Option<Self::Item> {
        if self.last == self.max {
            return None;
        }
        self.last += self.vec;
        Some(self.last.clone())
    }
}

fn parse_input(input: &str) -> Result<Vec<VentLine>> {
    input
        .replace("\r", "")
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<VentLine>())
        .collect()
}

fn count_collisions(vent_lines: &[VentLine]) -> Result<usize> {
    let mut pos_count = HashMap::<Pos, usize>::new();
    for vent_line in vent_lines {
        //debug!("Line: {}", vent_line);
        for pos in vent_line.iter_positions()? {
            //debug!("Pos: {}", pos);
            *pos_count.entry(pos).or_insert(0) += 1;
        }
    }

    Ok(pos_count.values().filter(|count| **count >= 2).count())
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let mut vent_lines = parse_input(input)?;
    vent_lines.drain_filter(|vl| vl.is_diagnonal());
    count_collisions(&vent_lines)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    count_collisions(&parse_input(input)?)
}
