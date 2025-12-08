use std::fmt;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug)]
struct Matrix {
    rows: Vec<Vec<char>>
}

impl Matrix {
    fn set(self: &mut Self, row: isize, column: isize, new_value: char) -> Option<()> {
        let _value = *self
            .rows
            .get(row as usize)?
            .get(column as usize)?;

        self.rows[row as usize][column as usize] = new_value;

        return Some(());
    }

    fn get(self: &Self, row: isize, column: isize) -> Option<MatrixElement> {
        let value = *self
            .rows
            .get(row as usize)?
            .get(column as usize)?;

        Some(MatrixElement {
            row: row,
            column: column,
            value: value
        })
    }

    fn remove_movable_rolls(self: &mut Self) {
        let to_remove_elements: Vec<MatrixElement> = self
            .into_iter()
            .filter(|element| {
                element.is_roll()
            })
            .map(|element| {
                let count = count_adyacent_rolls(self, &element);
                return (element, count);
            })
            .filter(|(_element, count)| {
                *count < 4
            })
            .map(|(element, _count)| {
                element
            })
            .collect();

        for element in to_remove_elements {
            self.set(element.row, element.column, '.');
        }
    }

    fn count_movable_rolls(self: &Self) -> usize {
        return self
            .into_iter()
            .filter(|element| {
                element.is_roll()
            })
            .map(|element| {
                count_adyacent_rolls(self, &element)
            })
            .filter(|n| {
                *n < 4
            })
            .count();
    }
}

struct MatrixElement {
    row: isize,
    column: isize,
    value: char,
}

impl MatrixElement {
    fn is_roll(self: &Self) -> bool {
        self.value == '@'
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.clone() {
            for element in row {
                _ = write!(f, "{} ", element);
            }
            _ = write!(f, "\n");
        }

        return Ok(());
    }
}

struct MatrixIterator<'a> {
    matrix: &'a Matrix,
    current_row: usize,
    current_column: usize,
}

impl<'a> IntoIterator for &'a Matrix {
    type Item = MatrixElement;
    type IntoIter = MatrixIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIterator {
            matrix: self,
            current_row: 0,
            current_column: 0,
        }
    }
}

impl<'a> Iterator for MatrixIterator<'a> {
    type Item = MatrixElement;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.current_row >= self.matrix.rows.len() {
                return None;
            }

            let row = &self.matrix.rows[self.current_row];

            if self.current_column < row.len() {
                let row_ = self.current_row;
                let column =  self.current_column;
                let value = row[self.current_column];

                self.current_column = self.current_column + 1;
                
                return Some(MatrixElement {
                    row: row_ as isize,
                    column: column as isize,
                    value: value,
                });
            }
            else {
                self.current_row = self.current_row + 1;
                self.current_column = 0;
            }
        }
        
    }
}

fn load_matrix(input: &str) -> Matrix {
    let rows = input
        .lines()
        .map(|line| {
            line.chars().collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();

    return Matrix {
        rows: rows
    };
}

fn count_adyacent_rolls(matrix: &Matrix, element: &MatrixElement) -> u8 {
    // horizontal
    // row + 1
    // row - 1 

    // vertical
    // col + 1
    // col - 1

    // diagonal
    // row + 1 && col + 1
    // row + 1 && col - 1
    // row - 1 && col - 1
    // row - 1 && col + 1

    let mut count = 0;
    let movements: [[isize; 2]; 8] = [
        [1, 0],
        [-1, 0],
        [0, 1],
        [0, -1],
        [1, 1],
        [1, -1],
        [-1, -1],
        [-1, 1]
    ];

    for [c, r] in movements {
        if let Some(element) = matrix.get(element.row + r, element.column + c) {
            if element.is_roll() {
                count = count + 1;
            }
        }
    }

    return count;
}

fn main() {
    let mut matrix = load_matrix(INPUT);

    let mut count = matrix.count_movable_rolls();
    let mut total_count = count;

    while count > 0 {
        matrix.remove_movable_rolls();
        count = matrix.count_movable_rolls();
        total_count = total_count + count;
    }

    println!("Count: {}", total_count);
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn test_count_movable_rolls() {
        let matrix = load_matrix(EXAMPLE_INPUT);
        let got = matrix.count_movable_rolls();
        let expected = 13;

        assert_eq!(got, expected, "{}", "Count all movable rolls from example input");
    }
    
    struct CoundAdyacentRollsTestCase<'a> {
        input: MatrixElement,
        expected_output: u8,
        description: &'a str
    }

    #[test]
    fn test_count_adyacents() {
        let matrix = load_matrix(EXAMPLE_INPUT);

        let test_cases = vec![
            CoundAdyacentRollsTestCase {
                input: MatrixElement {
                    row: 0,
                    column: 3,
                    value: '@',
                },
                expected_output: 3,
                description: "Counts adyacent rolls"
            }
        ];

        for test in test_cases {
            let got = count_adyacent_rolls(&matrix, &test.input);
            assert_eq!(got, test.expected_output, "{}", test.description);
        }
    }
}

