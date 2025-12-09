use std::cmp;
use std::num::ParseIntError;

use itertools::Itertools;

const INPUT: &str = include_str!("../input.txt");

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Range {
    start: u128,
    end: u128
}

#[derive(Debug)]
#[derive(PartialEq)]
struct State {
    ranges: Vec<Range>,
    ingredients: Vec<u128>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum LoadInputErrors<'a> {
    ParseNumberError,
    SplitRangesError(&'a str)
}

impl<'a> From<ParseIntError> for LoadInputErrors<'a> {
    fn from(_item: ParseIntError) -> Self {
        return LoadInputErrors::ParseNumberError{};
    }
}

fn load_input<'a>(input: &'a str) -> Result<State, LoadInputErrors<'a>> {
    let mut iterator = input.lines();
    let mut ranges: Vec<Range> = vec![];
    let mut ingredients: Vec<u128> = vec![];

    // Load ranges
    for line in &mut iterator {
        if line.len() <= 1 {
            break;
        }

        let Some((start, end)) = line.split_once("-")
            else {
                return Err(LoadInputErrors::SplitRangesError(line));
            };

        let start: u128 = start.parse()?;
        let end: u128 = end.parse()?;

        ranges.push(Range { start: start, end: end });
    }

    // Load IDs
    for line in &mut iterator {
        let ingredient = line.parse()?;
        ingredients.push(ingredient);
    }

    return Ok(State {
        ranges: ranges,
        ingredients: ingredients,
    });
}

fn is_fresh_ingredient(ranges: &[Range], ingredient: &u128) -> bool {
    for range in ranges {
        if (range.start..range.end+1).contains(ingredient) {
            return true;
        }
    }

    return false;
}

fn get_fresh_ingredients(input: &State) -> impl Iterator<Item=u128> {
    input
        .ingredients
        .iter()
        .filter(|&ingredient| {
            is_fresh_ingredient(&input.ranges, ingredient)
        })
        .map(|&ingredient| {
            ingredient
        })
}

// [
//     Range { start: 3, end: 5 },
//     Range { start: 10, end: 14 },
//     Range { start: 12, end: 18 },
//     Range { start: 16, end: 20 },
// ]
// => "Compressed" Ranges
// (3, 5)
// (10, 20)
fn count_fresh_ingredients(ranges: &[Range]) -> u128 {
    let merged_ranges = ranges
        .to_vec()
        .into_iter()
        .sorted_by_key(|r| r.start)
        .coalesce(|prev, curr| {
            if curr.start <= prev.end {
                Ok(Range { start: prev.start, end: cmp::max(prev.end, curr.end) })
            } else {
                Err((prev, curr))
            }
        });

    let mut count = 0;

    for range in merged_ranges {
        let dv = (range.end + 1) - range.start;
        count = count + dv;
    }
    
    return count;
}

fn main() -> Result<(), LoadInputErrors<'static>> {
    let state = load_input(INPUT)?;
    let fresh_ingredients = get_fresh_ingredients(&state);

    println!("Fresh ingredients: {}, Available Fresh ingredients: {}",
        fresh_ingredients.count(),
        count_fresh_ingredients(&state.ranges));

    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    struct IsFreshIngredientTestCase {
        input: (u128, Vec<Range>),
        expected_output: bool,
        description: &'static str,
    }
    
    #[test]
    fn test_is_fresh_ingredient() {
        let test_cases = vec![
            IsFreshIngredientTestCase {
                input: (5, vec![Range{ start: 1, end: 5 }]),
                expected_output: true,
                description: "It finds fresh ingredient"
            },
            IsFreshIngredientTestCase {
                input: (10, vec![Range{ start: 1, end: 5 }]),
                expected_output: false,
                description: "It finds spoiled ingredient"
            }
        ];

        for test in test_cases {
            let (test_input, test_ranges) = test.input;
            let got = is_fresh_ingredient(&test_ranges, &test_input);

            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }

    #[test]
    fn test_sort_ranges() {
        let expected = [
            Range { start: 1, end: 20 },
            Range { start: 1, end: 10 },
            Range { start: 2, end: 5 }
        ];
    }

    #[test]
    fn test_list_fresh_ingredients() {
        let expected = 14;

        let Ok(input) = load_input(EXAMPLE_INPUT)
            else { todo!(); };

        let got = count_fresh_ingredients(&input.ranges);

        assert_eq!(got, expected, "{}", "Correctly list all fresh ingredients");
    }
    
    #[test]
    fn test_load_input() {
        let expected = Ok(State {
            ranges: vec![
                Range { start: 3, end: 5 },
                Range { start: 10, end: 14 },
                Range { start: 16, end: 20 },
                Range { start: 12, end: 18},
            ],
            ingredients: vec![
                1,
                5,
                8,
                11,
                17,
                32
            ]
        });

        let got = load_input(EXAMPLE_INPUT);
        assert_eq!(got, expected, "{}", "It correctly parses input");
    }
}

