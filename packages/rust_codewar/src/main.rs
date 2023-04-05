use rand::{prelude, random, thread_rng, Rng};
use std::io;

fn main() {
    let arr = vec![1, 2, 3, 4];

    let result: Vec<i32> = arr.iter().step_by(2).cloned().collect();

    println!("result: {:#?}", result);

    fn invert(values: &[i32]) -> Vec<i32> {
        match values.is_empty() {
            true => vec![],
            false => values.iter().map(|value| -value).collect(),
        }
    }

    fn bool_to_word(value: bool) -> &'static str {
        if !value {
            return "No";
        }

        "Yes"
    }

    fn summation(n: i32) -> i32 {
        let mut count = 0;
        for value in 1..=n {
            count += value;
        }

        count
    }

    fn repeat_str(src: &str, count: usize) -> String {
        vec![src; count].concat()
    }

    fn even_or_odd(i: i32) -> &'static str {
        match (i % 2).abs() {
            0 => "Even",
            _ => "Odd",
        }
    }

    pub fn remove_char(s: &str) -> String {
        s[1..s.len() - 1].to_string()
    }

    fn make_upper_case(s: &str) -> String {
        s.to_uppercase()
    }

    fn string_to_array(s: &str) -> Vec<String> {
        s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
    }

    fn reverse_words(str: &str) -> String {
        str.split(" ")
            .map(|s| s.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }

    fn find_smallest_int(arr: &[i32]) -> i32 {
        *arr.iter().min().unwrap()
    }

    fn row_sum_odd_numbers(n: i64) -> i64 {
        n.pow(3)
    }

    fn is_triangle(a: i64, b: i64, c: i64) -> bool {
        a + b > c && a + c > b && b + c > a
    }

    fn cockroach_speed(s: f64) -> i64 {
        (s * 1000.0 / 36.0).floor() as i64
    }

    fn no_space(x: String) -> String {
        x.replace(" ", "")
    }

    fn dna_to_rna(dna: &str) -> String {
        dna.replace("T", "U")
    }

    fn min_max(lst: &[i32]) -> (i32, i32) {
        lst.iter().fold((lst[0], lst[1]), |(min, max), &current| {
            (
                if min > current { current } else { min },
                if max < current { current } else { max },
            )
        })
    }
}
