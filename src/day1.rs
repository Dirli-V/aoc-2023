use crate::{ParseResult, Solution, SolutionResult};

#[derive(Default)]
pub(crate) struct Day1 {
    text: Vec<Vec<char>>,
    text_replaced: Vec<Vec<char>>,
}

impl Solution for Day1 {
    fn parse(&mut self, input_lines: impl Iterator<Item = String>) -> ParseResult {
        let lines = input_lines.collect::<Vec<_>>();
        self.text = lines.iter().map(|line| line.chars().collect()).collect();
        self.text_replaced = lines
            .iter()
            .map(|line| line.replace("one", "1"))
            .map(|line| line.replace("two", "2"))
            .map(|line| line.replace("three", "3"))
            .map(|line| line.replace("four", "4"))
            .map(|line| line.replace("five", "5"))
            .map(|line| line.replace("six", "6"))
            .map(|line| line.replace("seven", "7"))
            .map(|line| line.replace("eight", "8"))
            .map(|line| line.replace("nine", "9"))
            .map(|line| line.chars().collect())
            .collect();

        Ok(())
    }

    fn solve1(&self) -> SolutionResult {
        let sum: u32 = self
            .text
            .iter()
            .map(|chars| {
                let first = chars.iter().find(|c| c.is_ascii_digit()).unwrap();
                let last = chars.iter().rev().find(|c| c.is_ascii_digit()).unwrap();

                first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
            })
            .sum();

        Ok(format!("{sum}"))
    }

    fn solve2(&self) -> SolutionResult {
        let sum: u32 = self
            .text_replaced
            .iter()
            .map(|chars| {
                let first = chars.iter().find(|c| c.is_ascii_digit()).unwrap();
                let last = chars.iter().rev().find(|c| c.is_ascii_digit()).unwrap();

                first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
            })
            .sum();

        Ok(format!("{sum}"))
    }

    fn file_name(&self) -> &'static str {
        "day1.txt"
    }
}

fn add_numbers(s: String) -> String {
    let chars = s.chars().collect::<Vec<_>>();
    let mut output = Vec::new();

    for i in 0..chars.len() {}
}
