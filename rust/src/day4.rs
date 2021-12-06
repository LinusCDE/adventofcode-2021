use anyhow::Result;
use std::result::Result as StdResult;

type BingoNumber = u8;

struct BingoField {
    number: BingoNumber,
    marked: bool,
}

impl From<BingoNumber> for BingoField {
    fn from(number: BingoNumber) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

impl std::fmt::Display for BingoField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut text = format!("{:2}", self.number);
        if self.marked {
            text = String::from_iter(text.chars().into_iter().map(|char| match char {
                '1' => '\u{2776}',
                '2' => '\u{2777}',
                '3' => '\u{2778}',
                '4' => '\u{2779}',
                '5' => '\u{277A}',
                '6' => '\u{277B}',
                '7' => '\u{277C}',
                '8' => '\u{277D}',
                '9' => '\u{277E}',
                '0' => '\u{1F10C}',
                /*'1' => '\u{2460}',
                '2' => '\u{2461}',
                '3' => '\u{2462}',
                '4' => '\u{2463}',
                '5' => '\u{2464}',
                '6' => '\u{2465}',
                '7' => '\u{2466}',
                '8' => '\u{2467}',
                '9' => '\u{2468}',
                '0' => '\u{24EA}',*/
                _ => char,
            }));
        }
        write!(f, "{}", text)
    }
}

struct BingoCard {
    grid: [[BingoField; 5]; 5],
    last_marked_number: Option<BingoNumber>,
}

impl BingoCard {
    pub fn from_lines(line_set: &[&str]) -> Result<Self> {
        let numbers = line_set
            .iter()
            .map(|l| l.replace("\n", " "))
            .flat_map(|l| l.split(" ").map(|n| n.to_owned()).collect::<Vec<_>>())
            .filter(|word| !word.is_empty())
            .map(|word| word.parse())
            .collect::<StdResult<Vec<BingoNumber>, _>>()?;

        ensure!(
            numbers.len() == 5 * 5,
            "Wrong amount of numbers for a 5x5 bingo card!"
        );

        Ok(Self {
            grid: [
                [
                    numbers[0 * 5 + 0].into(),
                    numbers[1 * 5 + 0].into(),
                    numbers[2 * 5 + 0].into(),
                    numbers[3 * 5 + 0].into(),
                    numbers[4 * 5 + 0].into(),
                ],
                [
                    numbers[0 * 5 + 1].into(),
                    numbers[1 * 5 + 1].into(),
                    numbers[2 * 5 + 1].into(),
                    numbers[3 * 5 + 1].into(),
                    numbers[4 * 5 + 1].into(),
                ],
                [
                    numbers[0 * 5 + 2].into(),
                    numbers[1 * 5 + 2].into(),
                    numbers[2 * 5 + 2].into(),
                    numbers[3 * 5 + 2].into(),
                    numbers[4 * 5 + 2].into(),
                ],
                [
                    numbers[0 * 5 + 3].into(),
                    numbers[1 * 5 + 3].into(),
                    numbers[2 * 5 + 3].into(),
                    numbers[3 * 5 + 3].into(),
                    numbers[4 * 5 + 3].into(),
                ],
                [
                    numbers[0 * 5 + 4].into(),
                    numbers[1 * 5 + 4].into(),
                    numbers[2 * 5 + 4].into(),
                    numbers[3 * 5 + 4].into(),
                    numbers[4 * 5 + 4].into(),
                ],
            ],
            last_marked_number: None,
        })
    }

    pub fn mark(&mut self, number: BingoNumber) {
        self.grid
            .iter_mut()
            .flat_map(|r| r.iter_mut())
            .filter(|f| f.number == number)
            .for_each(|f| f.marked = true);
        self.last_marked_number = Some(number);
    }

    pub fn is_bingo(&self) -> bool {
        'outer: for i in 0..5 {
            if self.grid[i].iter().all(|f| f.marked) {
                return true;
            }
            for j in 0..5 {
                if !self.grid[j][i].marked {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }

    pub fn calc_score(&self) -> Option<usize> {
        if !self.is_bingo() {
            return None;
        }
        let last_marked_number = self.last_marked_number? as usize;

        Some(
            self.grid
                .iter()
                .flat_map(|r| r.iter())
                .filter(|f| !f.marked)
                .map(|f| f.number as usize)
                .sum::<usize>()
                * last_marked_number,
        )
    }
}

impl std::fmt::Display for BingoCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..5 {
            write!(
                f,
                "{} {} {} {} {}\n",
                self.grid[0][y], self.grid[1][y], self.grid[2][y], self.grid[3][y], self.grid[4][y]
            )?;
        }
        write!(f, "Last Marked: {:?}", self.last_marked_number)
    }
}

fn parse_input(input: &str) -> Result<(Vec<BingoNumber>, Vec<BingoCard>)> {
    let input = input.replace("\r", ""); // Windows safety
    let line_sets = input
        .split("\n\n")
        .map(|s| s.split("\n").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let called_numbers = line_sets[0][0]
        .split(",")
        .map(|n| n.parse())
        .collect::<StdResult<Vec<BingoNumber>, _>>()?;

    let cards: Vec<BingoCard> = line_sets[1..]
        .iter()
        .map(|ls| BingoCard::from_lines(ls))
        .collect::<StdResult<Vec<BingoCard>, _>>()?;
    Ok((called_numbers, cards))
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let (called_numbers, mut cards) = parse_input(input)?;

    for called_number in called_numbers {
        for card in &mut cards {
            card.mark(called_number);
            if let Some(score) = card.calc_score() {
                debug!("Solved:\n{}", card);
                return Ok(score);
            }
        }
    }

    Err(anyhow!("No card ever won!"))
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let (called_numbers, mut cards) = parse_input(input)?;

    for called_number in called_numbers {
        cards.iter_mut().for_each(|card| card.mark(called_number));
        let drained = cards
            .drain_filter(|card| card.is_bingo())
            .collect::<Vec<_>>();
        if cards.is_empty() && drained.len() == 1 {
            return Ok(drained[0]
                .calc_score()
                .ok_or(anyhow!("Expected card to have a score!"))?);
        }
    }

    Err(anyhow!("Found no single card remaining!"))
}
