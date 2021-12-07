fn min_fuel<F>(input: &str, fuel_calc: F) -> anyhow::Result<usize>
where
    F: Fn(i16, i16) -> usize,
{
    let positions = input
        .trim()
        .split(',')
        .map(|word| word.parse())
        .collect::<Result<Vec<i16>, _>>()?;
    ensure!(positions.len() > 0, "No positions found!");

    // Iter from min to incl. max horizontal postition and find lowest
    // sum of all fuel calculations to that point.
    (*positions.iter().min().unwrap()..=*positions.iter().max().unwrap())
        .map(|t| positions.iter().map(|p| fuel_calc(*p, t)).sum())
        .min()
        .ok_or(anyhow!("No minimum fuel found!"))
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |p, t| (t - p).abs() as usize)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |p, t| (1..=(t - p).abs() as usize).sum())
}
