package main

import (
	"fmt"
	"bufio"
	"os"
	"log"
	"strings"
	"strconv"
)

type Rule [2]int
type Update []int

func index_of(values []int, x int) (int) {
	for i, value := range values {
		if value == x { return i }
	}
	return -1
}

func does_update_follow_rule(update Update, rule Rule) (bool) {
	i1 := index_of(update, rule[0])
	i2 := index_of(update, rule[1])
	if i1 < 0 || i2 < 0 { return true }
	return (i1 < i2)
}

func is_update_valid(update Update, rules []Rule) (bool) {
	for _, rule := range rules {
		if !does_update_follow_rule(update, rule) { return false }
	}
	return true
}

func rules_involving_number_first(number int, rules []Rule) ([]Rule) {
	var rules_with_number []Rule
	for _, rule := range rules {
		if rule[0] == number {
			rules_with_number = append(rules_with_number, rule)
		}
	}
	return rules_with_number
}

func rules_involving_number_second(number int, rules []Rule) ([]Rule) {
	var rules_with_number []Rule
	for _, rule := range rules {
		if rule[1] == number {
			rules_with_number = append(rules_with_number, rule)
		}
	}
	return rules_with_number
}

func rules_involving_number(number int, rules []Rule) ([]Rule) {
	return append(rules_involving_number_first(number, rules), rules_involving_number_second(number, rules)...)
}

func rules_involving_update(update Update, rules []Rule) ([]Rule) {
	var rules_for_update []Rule
	for _, rule := range rules {
		if index_of(update, rule[0]) != -1 && index_of(update, rule[1]) != -1 {
			rules_for_update = append(rules_for_update, rule)
		}
	}
	return rules_for_update
} 

func part_1(updates []Update, rules []Rule) {
	middle_pages_sum := 0
	for _, update := range updates {
		if is_update_valid(update, rules) {
			middle_pages_sum += update[(len(update)+1)/2 - 1]
		}
	}
	fmt.Println("Sum of middle pages =", middle_pages_sum)
}

func part_2(updates []Update, rules []Rule) {
	middle_pages_sum := 0
	for _, update := range updates {
		if !is_update_valid(update, rules) {
			sorted_update := make([]int, len(update), len(update))
			rules_for_update := rules_involving_update(update, rules)
			for _, page := range update {
				index := 0
				for _, rule := range rules_for_update {
					if rule[1] == page { index += 1 }
				}
				sorted_update[index] = page
			}
			middle_pages_sum += sorted_update[(len(update)+1)/2 - 1]
		}
	}
	fmt.Println("Sum of middle pages in sorted updates:", middle_pages_sum)
}

func main() {
	file, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	// Parsing input
	scanner := bufio.NewScanner(file)
	var rules []Rule
	var updates []Update
	for scanner.Scan() {
		line := scanner.Text()
		if strings.Contains(line, "|") {
			// Parsing rules X|Y
			values := strings.Split(line, "|")
			
			i1, err := strconv.Atoi(values[0])
			if err != nil {
				log.Fatal(err)
			}
			i2, err := strconv.Atoi(values[1])
			if err != nil {
				log.Fatal(err)
			}
			rule := [2]int{i1, i2}
			rules = append(rules, rule)
		} else if strings.Contains(line, ",") {
			// Parsing updates X1,X2,X3,...,XN
			values := strings.Split(line, ",")
			var update []int
			for _, value := range values {
				v, err := strconv.Atoi(value)
				if err != nil {
					log.Fatal(err)
				}
				update = append(update, v)
			}
			updates = append(updates, update)
		}
		if err := scanner.Err(); err != nil {
			log.Fatal(err)
		}
	}
	part_1(updates, rules)
	part_2(updates, rules)
}