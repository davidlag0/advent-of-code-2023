/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from") when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by a very young Elf who was apparently just excited to show off her art skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a specific calibration value that the Elves now need to recover. On each line, the calibration value can be found by combining the first digit and the last digit (in that order) to form a single two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?

--- Part Two ---

Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters: one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".

Equipped with this new information, you now need to find the real first and last digit on each line. For example:

two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.

What is the sum of all of the calibration values?
*/

use std::collections::BTreeMap;

fn calibration_value(raw_calibration: &str) -> u32 {
    let digits: Vec<char> = raw_calibration
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if !digits.is_empty() {
        let mut calibration_value_str = String::from(digits[0]);
        calibration_value_str.push(digits[digits.len() - 1]);

        match calibration_value_str.parse::<u32>() {
            Ok(parsed_int) => return parsed_int,
            Err(_err) => unreachable!(),
        }
    }

    0
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut sum_of_calibration_values = 0;

    for line in input.split('\n') {
        sum_of_calibration_values += calibration_value(line);
    }

    Ok(sum_of_calibration_values.to_string())
}

fn calibration_value_2(raw_calibration: &str) -> usize {
    let mut map: BTreeMap<usize, usize> = BTreeMap::new();
    const ENGLISH_NUMBERS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut calibration_value: usize = 0;

    // First word
    for pattern in ENGLISH_NUMBERS.iter().enumerate() {
        if let Some(position) = raw_calibration.find(pattern.1) {
            map.insert(position, pattern.0 + 1);
        }
    }

    // Last word
    for pattern in ENGLISH_NUMBERS.iter().enumerate() {
        if let Some(position) = raw_calibration.rfind(pattern.1) {
            map.insert(position, pattern.0 + 1);
        }
    }

    // First digit
    match raw_calibration
        .char_indices()
        .filter(|(_pos, char)| char.is_ascii_digit())
        .take(1)
        .next()
    {
        Some((position, character)) => match character.to_digit(10) {
            Some(digit) => map.insert(position, digit as usize),
            _ => unreachable!(),
        },
        _ => None,
    };

    // Last digit
    match raw_calibration
        .char_indices()
        .filter(|(_pos, char)| char.is_ascii_digit())
        .last()
    {
        Some((position, character)) => match character.to_digit(10) {
            Some(digit) => map.insert(position, digit as usize),
            _ => unreachable!(),
        },
        _ => None,
    };

    if let Some((_, value)) = map.first_key_value() {
        calibration_value += *value * 10
    }

    if let Some((_, value)) = map.last_key_value() {
        calibration_value += *value
    }

    calibration_value
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut sum_of_calibration_values = 0;

    for line in input.split('\n') {
        sum_of_calibration_values += calibration_value_2(line);
    }

    Ok(sum_of_calibration_values.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day1::{part1, part2};

    static TEST_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
abcdef";

    static TEST_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    static TEST_INPUT_3: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
abcdef
jcb82eightwond";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(142.to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_2), Ok(281.to_string()));
        assert_eq!(part2(TEST_INPUT_3), Ok(363.to_string()));
    }
}
