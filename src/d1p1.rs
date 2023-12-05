/*
--- Day 1: Trebuchet?! ---

Something is wrong with global snow production, and you've been selected to take a look. The Elves
have even given you a map; on it, they've used stars to mark the top fifty locations that are
likely to be having problems.

You've been doing this long enough to know that to restore snow operations, you need to check all
fifty stars by December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent
calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants one star.
Good luck!

You try to ask why they can't just use a weather machine ("not powerful enough") and where they're
even sending you ("the sky") and why your map looks mostly blank ("you sure ask a lot of
questions") and hang on did you just say the sky ("of course, where do you think snow comes from")
when you realize that the Elves are already loading you into a trebuchet ("please hold still, we
need to strap you in").

As they're making the final adjustments, they discover that their calibration document (your puzzle
input) has been amended by a very young Elf who was apparently just excited to show off her art
skills. Consequently, the Elves are having trouble reading the values on the document.

The newly-improved calibration document consists of lines of text; each line originally contained a
specific calibration value that the Elves now need to recover. On each line, the calibration value
can be found by combining the first digit and the last digit (in that order) to form a single
two-digit number.

For example:

1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet

In this example, the calibration values of these four lines are 12, 38, 15, and 77. Adding these
together produces 142.

Consider your entire calibration document. What is the sum of all of the calibration values?
*/

use anyhow::Result;

pub fn find_first_last_number(s: &str) -> Result<u32> {
    let chars: Vec<char> = s.chars().collect();
    let mut front = 0;
    let mut back = chars.len().saturating_sub(1);

    let mut first_digit = None;
    let mut last_digit = None;

    while first_digit.is_none() || last_digit.is_none() {
        if chars[front].is_ascii_digit() && first_digit.is_none() {
            first_digit = Some(chars[front]);
        }

        if chars[back].is_ascii_digit() && last_digit.is_none() {
            last_digit = Some(chars[back]);
        }

        if (first_digit.is_some() && last_digit.is_some()) || front >= back {
            break;
        }

        if first_digit.is_none() {
            front += 1;
        }
        if last_digit.is_none() {
            back = back.saturating_sub(1);
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Ok(format!("{}{}", first, last).parse::<u32>()?),
        (Some(single), None) | (None, Some(single)) if front >= back => {
            Ok(format!("{0}{0}", single).parse::<u32>()?)
        }
        _ => Ok(0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part1() {
        let input = r#"1abc2
           pqr3stu8vwx
           a1b2c3d4e5f
           treb7uchet"#;
        let parsed_input: Vec<&str> = input.lines().map(str::trim).collect();
        let mut sum = 0;
        for line in parsed_input {
            sum += find_first_last_number(line).unwrap();
        }
        assert_eq!(sum, 142);
    }

    #[test]
    fn input_part1() {
        let input = include_str!("input/d1p1.txt");
        let parsed_input: Vec<&str> = input.lines().map(str::trim).collect();
        let mut sum = 0;
        for line in parsed_input {
            let number = find_first_last_number(line).unwrap();
            println!("{} -> {}", line, number);
            sum += number;
        }
        assert_eq!(sum, 54081);
    }
}
