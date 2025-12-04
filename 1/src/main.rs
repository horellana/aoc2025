#[derive(Debug)]
#[derive(PartialEq)]
struct Dial {
    position: u8
}

impl Dial {
    fn new() -> Self {
        Self {
            position: 0
        }
    }
}

fn move_right(dial: Dial, steps: u8) -> Dial {
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

fn move_left(dial: Dial, steps: u8) -> Dial {
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

fn main() {
    println!("{:?}", Dial::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        input_dial: Dial,
        output_dial: Dial,
        steps: u8,
    }
    
    #[test]
    fn test_move_left() {
        let test_cases: [TestCase; 2] = [
            TestCase {
                input_dial: Dial { position: 99 },
                output_dial: Dial { position: 98},
                steps: 1,
            },
            TestCase {
                input_dial: Dial { position: 0 },
                output_dial: Dial { position: 99},
                steps: 1,
            },
        ];

        for test in test_cases {
            let got: Dial = move_left(test.input_dial, test.steps);
            assert_eq!(got, test.output_dial)

        }
    }

    #[test]
    fn test_move_right() {
        let test_cases: [TestCase; 2] = [
            TestCase {
                input_dial: Dial { position: 0 },
                output_dial: Dial { position: 1},
                steps: 1,
            },
            TestCase {
                input_dial: Dial { position: 99 },
                output_dial: Dial { position: 0},
                steps: 1,
            }
        ];

        for test in test_cases {
            let got: Dial = move_right(test.input_dial, test.steps);
            assert_eq!(got, test.output_dial)

        }
    }
}
