use std::io;

#[derive(Debug)]
enum DialErrors {
    FailedToReadStdin,
    UnexpectedInput(String)
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
struct Dial {
    position: u16
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum DialDirections {
    Left,
    Right
}

struct DialMovement {
    direction: DialDirections,
    steps: u16
}

fn move_dial(dial: Dial, direction: DialDirections, steps: u16) -> Dial {
    match direction {
        DialDirections::Right => {
            move_right(dial, steps)
        },
        DialDirections::Left => {
            move_left(dial, steps)
        }
    }
}

fn move_right(dial: Dial, steps: u16) -> Dial {
    let mut new_position = dial.position;

    for _i in 0..steps {
        if new_position == 99 {
            new_position = 0
        }
        else {
            new_position = new_position + 1
        }
    }

    return Dial {
        position: new_position
    }
}

fn move_left(dial: Dial, steps: u16) -> Dial {
    let mut new_position = dial.position;

    for _i in 0..steps {
        if new_position == 0 {
            new_position = 99
        }
        else {
            new_position = new_position - 1
        }
    }

    return Dial {
        position: new_position
    }
}

fn parse_line(input: String) -> Result<(DialDirections, u16), DialErrors>{
    let characters: Vec<char> = input.chars().collect();

    let steps: u16 = (&characters[1..])
        .into_iter()
        .collect::<String>()
        .parse()
        .map_err(|_e| { DialErrors::UnexpectedInput(input.clone())})?;

    if characters[0] == 'R' {
        return Ok((DialDirections::Right, steps))
    }
    else if characters[0] == 'L' {
        return Ok((DialDirections::Left, steps))
    }
    else {
        return Err(DialErrors::UnexpectedInput(input))
    }
}

fn count_zeros(movements: Vec<DialMovement>) -> u32 {
    let mut dial = Dial { position: 50 };
    let mut count: u32 = 0;

    for movement in movements {
        for _ in 0..movement.steps {
            dial = move_dial(dial, movement.direction, 1);

            if dial.position == 0 {
                count = count + 1;
            }
        }
    }

    return count;
}

fn main() -> Result<(), DialErrors> {
    let movements = io::stdin()
        .lines()
        .map(|line| {
            match line {
                Ok(line) => parse_line(line),
                Err(_) => Err(DialErrors::FailedToReadStdin),
            }
        })
        .map(|dial_movement_tuple| {
            match dial_movement_tuple {
                Ok((direction, steps)) => Ok(DialMovement { direction: direction, steps: steps }),
                Err(_) => Err(DialErrors::UnexpectedInput("".to_string()))
            }
        })
        .collect::<Result<Vec<DialMovement>, DialErrors>>()?;

    let count = count_zeros(movements);
    println!("{}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CountZeroTestCase {
        dial: Dial,
        input: Vec<DialMovement>,
        output: u32
    }

    #[test]
    fn test_count_zeros() {
        let test_cases = vec![
            CountZeroTestCase {
                dial: Dial { position: 50 },
                input: vec![
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 68
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 30
                    },
                    DialMovement {
                        direction: DialDirections::Right,
                        steps: 48 
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 5 
                    },
                    DialMovement {
                        direction: DialDirections::Right,
                        steps: 60 
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 55
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 1
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 99
                    },
                    DialMovement {
                        direction: DialDirections::Right,
                        steps: 14
                    },
                    DialMovement {
                        direction: DialDirections::Left,
                        steps: 82
                    }
                ],
                output: 6,
            },
        ];


        for test in test_cases {
            let got = count_zeros(test.input);
            assert_eq!(got, test.output)
        }
    }

    struct ParseLineTestCase {
        input: String,
        output_direction: DialDirections,
        output_steps: u16,
        description: &'static str,
    }
    
    #[test]
    fn test_parse_line() {
        let test_cases: [ParseLineTestCase; 5] = [
            ParseLineTestCase {
                input: "L513".to_string(),
                output_direction: DialDirections::Left,
                output_steps: 513,
                description: "Parse 'Left' with 3 digits",
            },
            ParseLineTestCase {
                input: "L46".to_string(),
                output_direction: DialDirections::Left,
                output_steps: 46,
                description: "Parse 'Left' with 2 digits",
            },
            ParseLineTestCase {
                input: "L1".to_string(),
                output_direction: DialDirections::Left,
                output_steps: 1,
                description: "Parse 'Left' with 1 digit",
            },
            ParseLineTestCase {
                input: "R46".to_string(),
                output_direction: DialDirections::Right,
                output_steps: 46,
                description: "Parse 'Right' with 2 digits",
            },
            ParseLineTestCase {
                input: "R1".to_string(),
                output_direction: DialDirections::Right,
                output_steps: 1,
                description: "Parse 'Right' with 1 digit",
            }
        ];

        for test in test_cases {
            let Ok((direction, steps)) = parse_line(test.input) else { todo!() };
            assert_eq!(direction, test.output_direction, "{}", test.description);
            assert_eq!(steps, test.output_steps, "{}", test.description);
        }
    }
    
    struct DialMovementTestCase {
        input_dial: Dial,
        output_dial: Dial,
        steps: u16,
        description: &'static str
    }
    
    #[test]
    fn test_move_left() {
        let test_cases: [DialMovementTestCase; 3] = [
            DialMovementTestCase {
                input_dial: Dial { position: 99 },
                output_dial: Dial { position: 98},
                steps: 1,
                description: "move to the left"
            },
            DialMovementTestCase {
                input_dial: Dial { position: 0 },
                output_dial: Dial { position: 99},
                steps: 1,
                description: "handle underflow",
            },
            DialMovementTestCase {
                input_dial: Dial { position: 5 },
                output_dial: Dial { position: 95},
                steps: 10,
                description: "handle underflow",
            },
        ];

        for test in test_cases {
            let got: Dial = move_left(test.input_dial, test.steps);
            assert_eq!(got, test.output_dial, "{}", test.description)

        }
    }

    #[test]
    fn test_move_right() {
        let test_cases: [DialMovementTestCase; 3] = [
            DialMovementTestCase {
                input_dial: Dial { position: 11 },
                output_dial: Dial { position: 19},
                steps: 8,
                description: "move to the right",
            },
            DialMovementTestCase {
                input_dial: Dial { position: 0 },
                output_dial: Dial { position: 1},
                steps: 1,
                description: "move to the right",
            },
            DialMovementTestCase {
                input_dial: Dial { position: 99 },
                output_dial: Dial { position: 0},
                steps: 1,
                description: "handle overflow"
            }
        ];

        for test in test_cases {
            let got: Dial = move_right(test.input_dial, test.steps);
            assert_eq!(got, test.output_dial, "{}", test.description)

        }
    }
}
