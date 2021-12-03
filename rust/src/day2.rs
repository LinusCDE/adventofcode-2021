pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let (mut horizontal, mut depth) = (0isize, 0isize);

    for line in input
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let value: isize = line.split(" ").last().ok_or(anyhow!("No value"))?.parse()?;
        if line.starts_with("forward") {
            horizontal += value;
        } else if line.starts_with("down") {
            depth += value;
        } else if line.starts_with("up") {
            depth -= value;
        } else {
            bail!("Unexpected command: {}", value)
        }
    }

    Ok(horizontal * depth)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let (mut horizontal, mut depth, mut aim) = (0isize, 0isize, 0isize);

    for line in input
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        let value: isize = line.split(" ").last().ok_or(anyhow!("No value"))?.parse()?;
        if line.starts_with("forward") {
            horizontal += value;
            depth += aim * value;
        } else if line.starts_with("down") {
            aim += value;
        } else if line.starts_with("up") {
            aim -= value;
        } else {
            bail!("Unexpected command: {}", value)
        }
    }

    Ok(horizontal * depth)
}
