package main

import (
	"strconv"
	"strings"
)

type Measurement struct {
	measurement int64
}

func solve_day1_part1(input string) (string, error) {
	lines := strings.Split(input, "\n")
	increasements := 0

	var last_measurement *Measurement = nil
	for i := 0; i < len(lines); i++ {
		line := strings.TrimSpace(lines[i])
		if len(line) == 0 {
			continue
		}
		measurement, err := strconv.ParseInt(line, 10, 0)
		if err != nil {
			//return "", fmt.Errorf("Failed to parse line %s to Int", line)
			return "", err
		}

		if last_measurement != nil && measurement > last_measurement.measurement {
			increasements++
		}
		last_measurement = &Measurement{measurement: measurement}
	}
	return strconv.FormatInt(int64(increasements), 10), nil
}

func solve_day1_part2(input string) (string, error) {
	lines := strings.Split(input, "\n")
	increasements := 0

	window := make([]*Measurement, 3)
	// Abusing Measurement as a "nilable" int
	var last_measurement_sum *Measurement = nil

	for i := 0; i < len(lines); i++ {
		line := strings.TrimSpace(lines[i])
		if len(line) == 0 {
			continue
		}
		measurement, err := strconv.ParseInt(line, 10, 0)
		if err != nil {
			//return "", fmt.Errorf("Failed to parse line %s to Int", line)
			return "", err
		}

		window[0] = window[1]
		window[1] = window[2]
		window[2] = &Measurement{measurement: measurement}

		if window[0] != nil && window[1] != nil && window[2] != nil {
			sum := window[0].measurement + window[1].measurement + window[2].measurement

			if last_measurement_sum != nil && sum > last_measurement_sum.measurement {
				increasements++
			}

			last_measurement_sum = &Measurement{measurement: sum}
		}
	}
	return strconv.FormatInt(int64(increasements), 10), nil
}
