pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let input = input.replace("\r", ""); // Windows safety

    let bit_count = input.chars().position(|c| c == '\n').unwrap();
    let mut bits = vec![(0usize, 0usize); bit_count];

    for line in input
        .split('\n')
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
    {
        for (bit, char) in line.chars().enumerate() {
            match char {
                '0' => bits[bit].0 += 1,
                '1' => bits[bit].1 += 1,
                _ => bail!("Unexpected char: {}", char),
            }
        }
    }

    let mut gamma: usize = 0;
    let mut epsilon: usize = 0;
    for bit in 0..bit_count {
        if bits[bit].0 > bits[bit].1 {
            epsilon |= 1 << ((bit_count - 1) - bit);
        } else {
            gamma |= 1 << ((bit_count - 1) - bit);
        }
    }
    Ok(gamma * epsilon)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    let input = input.replace("\r", ""); // Windows safety

    let bit_count = input.chars().position(|c| c == '\n').unwrap();
    let numbers: Vec<usize> = input
        .split("\n")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| usize::from_str_radix(l, 2).unwrap())
        .collect();

    // Find oxygen generator rating
    let mut oxygen_generator_rating: Option<usize> = None;
    let mut remaining_numbers = numbers.clone();
    for bit in 0..bit_count {
        let mut nums_ones = Vec::with_capacity(remaining_numbers.len());
        let mut nums_zeroes = Vec::with_capacity(remaining_numbers.len());

        for number in remaining_numbers {
            if number & (1 << ((bit_count - 1) - bit)) != 0 {
                // 1 at bit pos
                nums_ones.push(number);
            } else {
                nums_zeroes.push(number);
            }
        }

        remaining_numbers = if nums_ones.len() >= nums_zeroes.len() {
            nums_ones
        } else {
            nums_zeroes
        };

        if remaining_numbers.len() == 1 {
            oxygen_generator_rating = Some(remaining_numbers[0]);
        }
    }
    let oxygen_generator_rating =
        oxygen_generator_rating.ok_or(anyhow!("No oxygen generator rating found!"))?;

    // Find CO0 scrubber rating
    let mut co2_scrubber_rating: Option<usize> = None;
    let mut remaining_numbers = numbers.clone();

    for bit in 0..bit_count {
        let mut nums_ones = Vec::with_capacity(remaining_numbers.len());
        let mut nums_zeroes = Vec::with_capacity(remaining_numbers.len());

        for number in remaining_numbers {
            if number & (1 << ((bit_count - 1) - bit)) != 0 {
                // 1 at bit pos
                nums_ones.push(number);
            } else {
                nums_zeroes.push(number);
            }
        }

        remaining_numbers = if nums_zeroes.len() <= nums_ones.len() {
            nums_zeroes
        } else {
            nums_ones
        };

        if remaining_numbers.len() == 1 {
            co2_scrubber_rating = Some(remaining_numbers[0]);
        }
    }
    let co2_scrubber_rating =
        co2_scrubber_rating.ok_or(anyhow!("No CO2 scrubber rating found!"))?;

    Ok(oxygen_generator_rating * co2_scrubber_rating)
}
