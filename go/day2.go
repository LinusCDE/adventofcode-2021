package main

import (
	"fmt"
	"strconv"
	"strings"
)

func solve_day2_part1(input string) (string, error) {
	horizontal := 0
	depth := 0

	for _, line := range strings.Split(input, "\n") {
		line := strings.TrimSpace(line)
		if len(line) == 0 {
			continue
		}
		value, err := strconv.Atoi(strings.Split(line, " ")[1])
		if err != nil {
			return "", err
		}
		if strings.Index(line, "forward") == 0 {
			horizontal += value
		} else if strings.Index(line, "down") == 0 {
			depth += value
		} else if strings.Index(line, "up") == 0 {
			depth -= value
		} else {
			return "", fmt.Errorf("Unexpected command: %d", value)
		}
	}
	return strconv.FormatInt(int64(horizontal*depth), 10), nil
}

func solve_day2_part2(input string) (string, error) {
	horizontal := 0
	depth := 0
	aim := 0

	for _, line := range strings.Split(input, "\n") {
		line := strings.TrimSpace(line)
		if len(line) == 0 {
			continue
		}
		value, err := strconv.Atoi(strings.Split(line, " ")[1])
		if err != nil {
			return "", err
		}
		if strings.Index(line, "forward") == 0 {
			horizontal += value
			depth += aim * value
		} else if strings.Index(line, "down") == 0 {
			aim += value
		} else if strings.Index(line, "up") == 0 {
			aim -= value
		} else {
			return "", fmt.Errorf("Unexpected command: %d", value)
		}
	}
	return strconv.FormatInt(int64(horizontal*depth), 10), nil
}
