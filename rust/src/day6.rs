pub fn simulate(input: &str, days: usize) -> anyhow::Result<u64> {
    // Index: Days remaining
    // Value: Amount of lanternfish
    let mut fish_per_days_remain = [0u64; 9];

    // Add initial fish
    for days in input.trim().split(",").map(|word| word.parse::<usize>()) {
        fish_per_days_remain[days?] += 1;
    }

    // Cycle all the simulated days
    for _ in 0..days {
        let reproduce_count = fish_per_days_remain[0];

        // Lower all a day
        fish_per_days_remain[0] = fish_per_days_remain[1];
        fish_per_days_remain[1] = fish_per_days_remain[2];
        fish_per_days_remain[2] = fish_per_days_remain[3];
        fish_per_days_remain[3] = fish_per_days_remain[4];
        fish_per_days_remain[4] = fish_per_days_remain[5];
        fish_per_days_remain[5] = fish_per_days_remain[6];
        fish_per_days_remain[6] = fish_per_days_remain[7] + reproduce_count; // Removed fish from 0, rentering at 6 days
        fish_per_days_remain[7] = fish_per_days_remain[8];
        fish_per_days_remain[8] = reproduce_count; // Newborn fish
    }

    Ok(fish_per_days_remain.iter().sum::<u64>())
}

pub fn solve_part_1(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input, 80)
}

pub fn solve_part_2(input: &str) -> anyhow::Result<impl std::fmt::Display> {
    simulate(input, 256)
}
