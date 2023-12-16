/*
--- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
*/

struct Engine<'a> {
    line_length: usize,
    data: &'a [u8],
}

impl<'a> Engine<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        Ok(Self {
            line_length: match input.find('\n') {
                Some(length) => length,
                None => return Err("Input has no new line character, exiting...".to_string()),
            },
            data: input.as_bytes(),
        })
    }
}

fn check_bounds(engine: &Engine, index_to_verify: usize) -> Option<usize> {
    if index_to_verify < engine.data.len() {
        return Some(index_to_verify);
    }

    None
}

fn indices_to_check(engine: &Engine, index: usize) -> [Option<usize>; 8] {
    [
        // index - engine.line_length - 1
        match index.checked_sub(engine.line_length) {
            Some(valid_index) => valid_index.checked_sub(2),
            None => None,
        },
        // index - engine.line_length
        match index.checked_sub(engine.line_length) {
            Some(valid_index) => valid_index.checked_sub(1),
            None => None,
        },
        // index - engine.line_length + 1
        index.checked_sub(engine.line_length),
        // index - 1
        index.checked_sub(1),
        // index + 1
        check_bounds(engine, index + 1),
        // index + engine.line_length - 1
        check_bounds(engine, index + engine.line_length),
        // index + engine.line_length
        check_bounds(engine, index + engine.line_length + 1),
        // index + engine.line_length + 1
        check_bounds(engine, index + engine.line_length + 2),
    ]
}

fn is_symbol_around(engine: &Engine, index: usize) -> bool {
    let indices_to_check_for_symbol = indices_to_check(engine, index);

    for index_to_check in indices_to_check_for_symbol {
        match index_to_check {
            Some(data_index) => match engine.data[data_index] {
                b'0'..=b'9' => continue,
                b'.' => continue,
                b'\n' => continue,
                _ => return true,
            },
            None => continue,
        }
    }

    false
}

pub fn part1(input: &str) -> Result<String, String> {
    let engine = match Engine::new(input) {
        Ok(engine) => engine,
        Err(error) => return Err(error),
    };

    let mut sum_of_part_numbers: i64 = 0;

    let mut number_first_index: usize = usize::MAX;
    let mut number_last_index: usize = usize::MAX;
    let mut found_symbol_around: bool = false;

    for (index, byte) in engine.data.iter().enumerate() {
        match byte {
            b'0'..=b'9' => {
                if number_first_index == usize::MAX {
                    number_first_index = index;
                    number_last_index = index;
                } else {
                    number_last_index = index;
                }

                if is_symbol_around(&engine, index) {
                    found_symbol_around = true;
                }
            }
            _ => {
                if number_first_index != usize::MAX && number_last_index != usize::MAX {
                    let potential_part_number = String::from_utf8_lossy(
                        &engine.data[number_first_index..number_last_index + 1],
                    );

                    match potential_part_number.parse::<i64>() {
                        Ok(part_number) => {
                            if found_symbol_around {
                                sum_of_part_numbers += part_number;
                            }
                        }
                        Err(error) => {
                            return Err(format!(
                                "Couldn't parse {:?} into u32: {:?}",
                                potential_part_number, error
                            ))
                        }
                    }

                    number_first_index = usize::MAX;
                    number_last_index = usize::MAX;
                    found_symbol_around = false;
                }
            }
        }
    }

    Ok(sum_of_part_numbers.to_string())
}

fn part_number(engine: &Engine, index: usize) -> Option<i64> {
    let mut part_number_low_index: usize = index;
    let mut part_number_high_index: usize = index;

    if !engine.data[index].is_ascii_digit() {
        return None;
    }

    while engine.data[part_number_low_index].is_ascii_digit() {
        part_number_low_index = match part_number_low_index.checked_sub(1) {
            Some(valid_index) => valid_index,
            None => break,
        }
    }

    if part_number_low_index != 0 {
        part_number_low_index += 1;
    }

    while engine.data[part_number_high_index].is_ascii_digit() {
        part_number_high_index = match part_number_high_index.checked_add(1) {
            Some(valid_index) => match check_bounds(engine, valid_index) {
                Some(still_valid_index) => still_valid_index,
                None => break,
            },
            None => break,
        }
    }

    if part_number_high_index != engine.data.len() {
        part_number_high_index -= 1;
    }

    match String::from_utf8_lossy(&engine.data[part_number_low_index..=part_number_high_index])
        .parse::<i64>()
    {
        Ok(part_number) => Some(part_number),
        Err(_) => None,
    }
}

fn part_numbers_around(engine: &Engine, index: usize) -> Vec<i64> {
    let mut part_numbers: Vec<i64> = Vec::new();

    let indices_to_check_for_part_number = indices_to_check(engine, index);

    for index_to_check in indices_to_check_for_part_number {
        match index_to_check {
            Some(data_index) => match engine.data[data_index] {
                b'0'..=b'9' => {
                    match part_number(engine, data_index) {
                        Some(valid_part_number) => {
                            if !part_numbers.contains(&valid_part_number) {
                                part_numbers.push(valid_part_number);
                            }
                        }
                        None => continue,
                    };
                }
                _ => continue,
            },
            None => continue,
        }
    }

    part_numbers
}

pub fn part2(input: &str) -> Result<String, String> {
    let engine = match Engine::new(input) {
        Ok(engine) => engine,
        Err(error) => return Err(error),
    };

    let mut sum_of_gear_ratios: i64 = 0;

    for (index, byte) in engine.data.iter().enumerate() {
        if byte == &b'*' {
            let part_numbers_around_potential_gear = part_numbers_around(&engine, index);

            if part_numbers_around_potential_gear.len() > 1 {
                sum_of_gear_ratios += part_numbers_around_potential_gear.iter().product::<i64>();
            }
        }
    }

    Ok(sum_of_gear_ratios.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day3::{
        check_bounds, is_symbol_around, part1, part2, part_number, part_numbers_around, Engine,
    };

    static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.0
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(4361.to_string()));
    }

    #[test]
    fn test_check_bounds() {
        let engine = Engine::new(TEST_INPUT).unwrap();
        assert_eq!(check_bounds(&engine, 45), Some(45));
    }

    #[test]
    fn test_is_symbol_around() {
        let engine = Engine::new(TEST_INPUT).unwrap();
        assert!(!is_symbol_around(&engine, 0));
        assert!(!is_symbol_around(&engine, 1));
        assert!(is_symbol_around(&engine, 2));
        assert!(is_symbol_around(&engine, 3));
        assert!(is_symbol_around(&engine, 4));
        assert!(!is_symbol_around(&engine, 5));
        assert!(!is_symbol_around(&engine, 6));
        assert!(!is_symbol_around(&engine, 7));
        assert!(!is_symbol_around(&engine, 8));
        assert!(!is_symbol_around(&engine, 9));
        assert!(is_symbol_around(&engine, 104));
    }

    #[test]
    fn test_part_number() {
        let engine = Engine::new(TEST_INPUT).unwrap();
        assert_eq!(part_number(&engine, 1), Some(467));
        assert_eq!(part_number(&engine, 4), None);
        assert_eq!(part_number(&engine, 5), Some(114));
        assert_eq!(part_number(&engine, 5), Some(114));
        assert_eq!(part_number(&engine, 108), Some(0));
    }

    #[test]
    fn test_part_numbers_around() {
        let engine = Engine::new(TEST_INPUT).unwrap();
        assert_eq!(part_numbers_around(&engine, 14), vec![467, 35]);
        assert_eq!(part_numbers_around(&engine, 47), vec![617]);
        assert_eq!(part_numbers_around(&engine, 93), vec![755, 598]);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(467835.to_string()));
    }
}
