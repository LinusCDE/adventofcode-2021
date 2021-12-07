fn min_fuel<F>(input: &str, fuel_calc: F) -> anyhow::Result<isize>
where
    F: Fn(isize) -> isize,
{
    let positions = input
        .trim()
        .split(',')
        .map(|word| word.parse())
        .collect::<Result<Vec<isize>, _>>()?;
    ensure!(positions.len() > 0, "No positions found!");

    // Iter from min to incl. max horizontal postition and find lowest
    // sum of all fuel calculations to that point.
    (*positions.iter().min().unwrap()..=*positions.iter().max().unwrap())
        .map(|t| positions.iter().map(|p| fuel_calc((t - *p).abs())).sum())
        .min()
        .ok_or(anyhow!("No minimum fuel found!"))
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |distance| distance)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |distance| distance * (distance + 1) / 2)
}
