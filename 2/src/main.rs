use std::io;
use std::num::{ParseIntError};

#[derive(Debug)]
#[derive(PartialEq)]
struct Range {
    start: u64,
    end: u64
}

#[derive(Debug)]
struct ParseRangeSplitError {
}

#[derive(Debug)]
enum ParseRangeError {
    SplitError(ParseRangeSplitError),
    InputTextError(ParseIntError),
}

impl From<ParseIntError> for ParseRangeError {
    fn  from(item: ParseIntError) -> Self {
        ParseRangeError::InputTextError(item)
    }
}

#[derive(Debug)]
enum ParseRangeListError {
    IoError(std::io::Error)
}

impl From<std::io::Error> for ParseRangeListError {
    fn from(error: std::io::Error) -> Self {
        ParseRangeListError::IoError(error)
    }
}

fn parse_range(input: &str) -> Result<Range, ParseRangeError> {
    let parts = input.split_once("-");

    match parts {
        Some((start_str, end_str)) => {
            Ok(Range {
                start: start_str.parse::<u64>()?,
                end: end_str.parse::<u64>()?,
            })
        },
        None => {
            Err(ParseRangeError::SplitError(ParseRangeSplitError{}))
        },
    }
}

fn parse_range_list(input: &str) -> Result<Vec<String>, ParseRangeListError> {
    Ok(input
        .split(",")
        .map(|s| { s.to_string() })
        .collect::<Vec<String>>())
}

fn is_invalid_id(input: &str) -> bool {
    let bytes = input.as_bytes();

    if bytes.len() % 2 != 0 {
        return false;
    }

    return bytes[0..bytes.len() / 2] == bytes[bytes.len() / 2..bytes.len()];
}

fn scan_range(range: Range) -> impl Iterator<Item=u64> {
    (range.start..range.end + 1) 
        .filter(|n| {
            is_invalid_id(&n.to_string()) 
        })
}

#[derive(Debug, Clone)]
enum FindInvalidIdsError {
    ParseListError,
    ParseRangeError,
}

impl From<ParseRangeError> for FindInvalidIdsError {
    fn from(_: ParseRangeError) -> Self {
        FindInvalidIdsError::ParseRangeError
    }
}

fn main() -> Result<(), FindInvalidIdsError> {
    let Ok(range_expressions): Result<Vec<String>, ParseRangeListError> = io::stdin()
        .lines()
        .try_fold(Vec::new(), |mut acc, line| {
            let range = parse_range_list(line?.as_str())?;
            acc.extend(range);
            return Ok(acc);
        })
        else {
            return Err(FindInvalidIdsError::ParseListError);
        };

    // println!("{:?}", range_expressions);
    
    let sum = range_expressions
        .iter()
        .map(|expression| {
            return parse_range(expression);
        })
        .map(|range| {
            let ids = scan_range(range?);
            return Ok::<_, FindInvalidIdsError>(ids);
        })
        .try_fold(0, |acc, invalid_ids| {
            let sum: u64 = invalid_ids?.sum();
            return Ok::<_, FindInvalidIdsError>(acc + sum);
        });

    println!("SUM: {}", sum?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ScanRangeTestCase {
        input: Range,
        expected_output: Vec<u64>,
        description: &'static str,
    }

    #[test]
    fn test_scan_range() {
        let test_cases = vec![
            ScanRangeTestCase {
                input: Range { start: 11, end: 22 },
                expected_output: vec![11, 22],
                description: "11-22"
            },
            ScanRangeTestCase {
                input: Range { start: 95, end: 115 },
                expected_output: vec![99],
                description: "95-115"
            },
            ScanRangeTestCase {
                input: Range { start: 998, end: 1012 },
                expected_output: vec![1010],
                description: "998-1012"
            },
            ScanRangeTestCase {
                input: Range { start: 1188511880, end: 1188511890 },
                expected_output: vec![1188511885],
                description: "1188511880-1188511890"
            },
            ScanRangeTestCase {
                input: Range { start: 222220, end: 222224 },
                expected_output: vec![222222],
                description: "222220-222224"
            },
            ScanRangeTestCase {
                input: Range { start: 446443, end: 446449 },
                expected_output: vec![446446],
                description: "446443-446449"
            },
            ScanRangeTestCase {
                input: Range { start: 38593856, end: 38593862 },
                expected_output: vec![38593859],
                description: "38593856-38593862"
            },
            ScanRangeTestCase {
                input: Range { start: 565653, end: 565659 },
                expected_output: vec![],
                description: "565653-565659"
            }
        ];

        for test in test_cases {
            let got = scan_range(test.input).collect::<Vec<u64>>();
            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }

    struct InvalidIdTestCase {
        input: String,
        expected_output: bool,
        description: &'static str,
    }
    
    #[test]
    fn test_is_invalid_id() {
        let test_cases = vec![
            InvalidIdTestCase {
                input: 11.to_string(),
                expected_output: true,
                description: "11 is not a valid id"
            },
            InvalidIdTestCase {
                input: 22.to_string(),
                expected_output: true,
                description: "22 is not a valid id"
            },
            InvalidIdTestCase {
                input: 222222.to_string(),
                expected_output: true,
                description: "222222 is not a valid id"
            },
            InvalidIdTestCase {
                input: 1188511885.to_string(),
                expected_output: true,
                description: "1188511885 is not a valid id"
            },
            InvalidIdTestCase {
                input: 38593859.to_string(),
                expected_output: true,
                description: "38593859 is not a valid id"
            },
        ];

        for test in test_cases {
            let got = is_invalid_id(&test.input);
            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }
    
    struct ParseRangeTestCase {
        input: &'static str,
        expected_output: Range,
        description: &'static str,
    }
    
    #[test]
    fn test_parse_range() {
        let test_cases = vec![
            ParseRangeTestCase {
                input: "24-46",
                expected_output: Range { start: 24, end: 46 },
                description: "Can parse ranges 1"
            },
            ParseRangeTestCase {
                input: "124420-259708",
                expected_output: Range { start: 124420, end: 259708 },
                description: "Can parse ranges 2"
            },
            ParseRangeTestCase {
                input: "99828221-99856128",
                expected_output: Range { start: 99828221, end: 99856128 },
                description: "Can parse ranges 3"
            },
            ParseRangeTestCase {
                input: "6868562486-6868811237",
                expected_output: Range { start: 6868562486, end: 6868811237 },
                description: "Can parse ranges 4"
            }
        ];

        for test in test_cases {
            let Ok(got) = parse_range(test.input)
                else { todo!() };

            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }

    struct ParseRangeListTestCase<'a> {
        input: &'static str,
        expected_output: Vec<&'a str>,
        description: &'static str,
    }

    #[test]
    fn test_parse_range_list() {
        let test_cases = vec![
            ParseRangeListTestCase {
                input: "24-46,124420-259708,584447-720297",
                expected_output: vec!["24-46", "124420-259708", "584447-720297"],
                description: "Parse range list by comma"
            }
        ];

        for test in test_cases {
            let Ok(got) = parse_range_list(test.input)
                else { todo!() };

            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }
}
