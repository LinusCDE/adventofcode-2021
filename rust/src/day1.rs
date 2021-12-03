pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let mut increments: usize = 0;
    let mut last_measurement: Option<u64> = None;

    for line in input.split('\n').map(|s| s.trim()) {
        if line.is_empty() {
            continue;
        }
        let measurement = line.parse()?;

        if let Some(last_measurement) = last_measurement {
            if measurement > last_measurement {
                increments += 1;
            }
        }
        last_measurement = Some(measurement);
    }
    Ok(increments)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let mut increments: usize = 0;
    let mut last_measurement_sum: Option<u64> = None;
    let mut window = (None::<u64>, None::<u64>, None::<u64>);

    for line in input.split('\n').map(|s| s.trim()) {
        if line.is_empty() {
            continue;
        }
        let measurement = line.parse()?;

        window.0 = window.1;
        window.1 = window.2;
        window.2 = Some(measurement);

        let measurement_sum = match window {
            (Some(val1), Some(val2), Some(val3)) => Some(val1 + val2 + val3),
            _ => None,
        };

        if_chain! {
            if let Some(last_measurement_sum) = last_measurement_sum;
            if let Some(measurement_sum) = measurement_sum;
            if measurement_sum > last_measurement_sum;
            then {
                increments += 1;
            }
        }
        last_measurement_sum = measurement_sum;
    }
    Ok(increments)
}
