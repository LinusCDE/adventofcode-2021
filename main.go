package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"

	"github.com/thatisuday/commando"
)

var _ = os.Args
var _ = fmt.Println
var _ = commando.Register

func main() {
	commando.
		Register(nil).
		AddArgument("day", "Which day to run", "").
		AddArgument("part", "Which part to run", "").
		AddFlag("input-file,f", "Which input file to use", commando.String, "DEFAULT").
		SetAction(func(args map[string]commando.ArgValue, flags map[string]commando.FlagValue) {
			day, err := strconv.ParseInt(args["day"].Value, 10, 0)
			if err != nil {
				fmt.Println("Invalid day:", args["day"].Value)
				return
			}
			if day < 1 || day > 31 {
				fmt.Println("Day must be between 1 and 31!")
				return
			}
			part, err := strconv.ParseInt(args["part"].Value, 10, 0)
			if err != nil {
				fmt.Println("Invalid part:", args["part"].Value)
				fmt.Println(err)
				return
			}
			if part != 1 && part != 2 {
				fmt.Println("Part can either be 1 or 2!")
				return
			}
			fmt.Println(day, part)

			filename, err := flags["input-file"].GetString()
			if err != nil {
				fmt.Println("Invalid input file!")
				fmt.Println(err)
				return
			}
			if filename == "DEFAULT" {
				filename = "input/day" + strconv.FormatInt(day, 10) + ".txt"
			}

			fmt.Println("Using file \"" + filename + "\"")

			//for k, v := range args {
			//	fmt.Printf("arg -> %v: %v(%T)\n", k, v.Value, v.Value)
			//}

			// print flags
			//for k, v := range flags {
			//	fmt.Printf("flag -> %v: %v(%T)\n", k, v.Value, v.Value)
			//}

			rawfilecontent, err := ioutil.ReadFile(filename)
			if err != nil {
				fmt.Println("Failed to read file \"" + filename + "\"")
				fmt.Println(err)
				return
			}
			filecontent := string(rawfilecontent[:])

			result := ""
			var result_err error = nil

			if day == 1 && part == 1 {
				result, result_err = solve_day1_part1(filecontent)
			} else if day == 1 && part == 2 {
				result, result_err = solve_day1_part2(filecontent)
			} else {
				fmt.Printf("Day %d part %d is not implemented, yet.\n", day, part)
				return
			}

			if result_err != nil {
				fmt.Println("Failed to solve!")
				fmt.Println(result_err)
				return
			}
			fmt.Println("Solved:", result)

		})
	commando.Parse(nil)
}
