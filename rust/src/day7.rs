fn min_fuel<F>(input: &str, fuel_calculation: F) -> anyhow::Result<usize>
where
    F: Fn(i16, i16) -> usize,
{
    let positions = input
        .trim()
        .split(",")
        .map(|word| word.parse())
        .collect::<Result<Vec<i16>, _>>()?;
    ensure!(positions.len() > 0, "No positions found!");

    let mut min_fuel = None;
    for target in *positions.iter().min().unwrap()..=*positions.iter().max().unwrap() {
        let mut fuel: usize = 0;
        for position in positions.iter().map(|p| *p) {
            fuel += fuel_calculation(position, target);
        }
        if let Some(min_fuel_val) = min_fuel {
            if fuel < min_fuel_val {
                min_fuel = Some(fuel);
            }
        } else {
            min_fuel = Some(fuel);
        }
    }

    min_fuel.ok_or(anyhow!("No minimum fuel found!"))
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |position, target| {
        (target as i16 - position as i16).abs() as usize
    })
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    min_fuel(input, |position, target| {
        // Is there an equation to do this??
        let mut fuel = 0;
        for dist in 0..((target as i16 - position as i16).abs() as usize) {
            fuel += dist + 1;
        }
        fuel
    })
}
