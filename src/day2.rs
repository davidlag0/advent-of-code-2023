/*
--- Day 2: Cube Conundrum ---

You're launched high into the atmosphere! The apex of your trajectory just barely reaches the surface of a large island floating in the sky. You gently land in a fluffy pile of leaves. It's quite cold, but you don't see much snow. An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow. He'll be happy to explain the situation, but it's a bit of a walk, so you have some time. They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue. Each time you play this game, he will hide a secret number of cubes of each color in the bag, and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag, grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input). Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again). The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes; the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with that configuration. However, game 3 would have been impossible because at one point the Elf showed you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?

--- Part Two ---

The Elf says they've stopped producing snow because they aren't getting any water! He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself. It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes. If any color had even one fewer cube, the game would have been impossible.
    Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
    Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
    Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together. The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
*/

const MAX_RED_CUBES: i64 = 12;
const MAX_GREEN_CUBES: i64 = 13;
const MAX_BLUE_CUBES: i64 = 14;

enum CubeColor {
    Red,
    Green,
    Blue,
}

struct Cube {
    count: i64,
    color: CubeColor,
}

impl Cube {
    pub fn new(cube_string: &str) -> Result<Self, String> {
        let cube_vec = cube_string.split_whitespace().collect::<Vec<&str>>();

        Ok(Self {
            count: match cube_vec[0].parse::<i64>() {
                Ok(value) => value,
                Err(_) => {
                    return Err(format!(
                        "Unable to parse cube string's number: {:?}",
                        cube_vec[0]
                    ))
                }
            },
            color: match cube_vec[1] {
                "red" => CubeColor::Red,
                "green" => CubeColor::Green,
                "blue" => CubeColor::Blue,
                _ => {
                    return Err(format!(
                        "Unable to parse cube string's color: {:?}",
                        cube_vec[1]
                    ))
                }
            },
        })
    }
}

fn is_round_possible(game: &str) -> bool {
    for cube in game
        .split(',')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()
    {
        let cube_vec = cube.split(' ').collect::<Vec<&str>>();
        let cube_number = cube_vec[0].parse::<i64>().unwrap_or(0);

        match cube_vec[1] {
            "red" => {
                if cube_number > MAX_RED_CUBES {
                    return false;
                }
            }
            "green" => {
                if cube_number > MAX_GREEN_CUBES {
                    return false;
                }
            }
            "blue" => {
                if cube_number > MAX_BLUE_CUBES {
                    return false;
                }
            }
            _ => {}
        }
    }
    true
}

fn game_rounds(line: &str) -> impl Iterator<Item = &str> {
    return line
        .split(':')
        .map(|line| line.trim())
        .collect::<Vec<&str>>()[1]
        .split(';')
        .map(|line| line.trim());
}

pub fn part1(input: &str) -> Result<String, String> {
    let mut sum_of_possible_game_ids: i64 = 0;

    for line in input.split_terminator('\n') {
        let game_id = line.split(':').collect::<Vec<&str>>()[0]
            .split(' ')
            .collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap_or(0);

        let mut possible_game = true;

        for round in game_rounds(line) {
            possible_game = is_round_possible(round);

            if !possible_game {
                break;
            }
        }

        if possible_game {
            sum_of_possible_game_ids += game_id;
        }
    }

    Ok(sum_of_possible_game_ids.to_string())
}

fn power_of_set_of_cubes<'a>(rounds: impl Iterator<Item = &'a str>) -> i64 {
    let mut min_red_cubes: i64 = 0;
    let mut min_green_cubes: i64 = 0;
    let mut min_blue_cubes: i64 = 0;

    for round in rounds {
        for single_cube in round.split(',').map(|line| line.trim()) {
            match Cube::new(single_cube) {
                Ok(cube) => match cube.color {
                    CubeColor::Red => {
                        if cube.count > min_red_cubes {
                            min_red_cubes = cube.count
                        }
                    }
                    CubeColor::Green => {
                        if cube.count > min_green_cubes {
                            min_green_cubes = cube.count
                        }
                    }
                    CubeColor::Blue => {
                        if cube.count > min_blue_cubes {
                            min_blue_cubes = cube.count
                        }
                    }
                },
                Err(_) => continue,
            };
        }
    }

    min_red_cubes * min_green_cubes * min_blue_cubes
}

pub fn part2(input: &str) -> Result<String, String> {
    let mut sum_of_powers: i64 = 0;

    for line in input.split_terminator('\n') {
        sum_of_powers += power_of_set_of_cubes(game_rounds(line))
    }

    Ok(sum_of_powers.to_string())
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    static TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    static TEST_INPUT_WITH_INVALID_GAME_IDS: &str =
        "Game x: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game x: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    static TEST_INPUT_WITH_INVALID_CUBE_COUNT: &str =
        "Game 3: 8 green, 6 blue, XX red; 5 blue, 4 red, 13 green; 5 green, 1 red";

    static TEST_INPUT_WITH_INVALID_CUBE_COLOR: &str =
        "Game 3: 8 green, 6 blue, 20 purple; 5 blue, 4 red, 13 green; 5 green, 1 red";

    static TEST_INPUT_WITH_TOO_MANY_GREEN_CUBES: &str =
        "Game 3: 8 green, 6 blue, 6 red; 5 blue, 4 red, 15 green; 5 green, 1 red";

    static TEST_INPUT_INVALID_CUBE_COUNT: &str =
        "Game 1: x blue, 4 red; 1 red, 2 green, 6 blue; 2 green";

    static TEST_INPUT_INVALID_CUBE_COLOR: &str =
        "Game 1: 3 blue, 4 purple; 1 red, 2 green, 6 blue; 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), Ok(8.to_string()));
    }

    #[test]
    fn test_part1_with_invalid_game_ids() {
        assert_eq!(part1(TEST_INPUT_WITH_INVALID_GAME_IDS), Ok(2.to_string()));
    }

    #[test]
    fn test_part1_with_invalid_cube_count() {
        assert_eq!(part1(TEST_INPUT_WITH_INVALID_CUBE_COUNT), Ok(3.to_string()));
    }

    #[test]
    fn test_part1_with_invalid_cube_color() {
        assert_eq!(part1(TEST_INPUT_WITH_INVALID_CUBE_COLOR), Ok(3.to_string()));
    }

    #[test]
    fn test_part1_with_too_many_green_cubes() {
        assert_eq!(
            part1(TEST_INPUT_WITH_TOO_MANY_GREEN_CUBES),
            Ok(0.to_string())
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), Ok(2286.to_string()));
    }

    #[test]
    fn test_part2_invalid_value_for_cube_count() {
        assert_eq!(part2(TEST_INPUT_INVALID_CUBE_COUNT), Ok(48.to_string()));
    }

    #[test]
    fn test_part2_invalid_value_for_cube_color() {
        assert_eq!(part2(TEST_INPUT_INVALID_CUBE_COLOR), Ok(12.to_string()));
    }
}
