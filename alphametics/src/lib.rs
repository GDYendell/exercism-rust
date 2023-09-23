use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let addend_re: Regex = Regex::new(r"[A-Za-z]+").unwrap();
    let space_re: Regex = Regex::new(r"\s").unwrap();
    let max_digit_count = |expression: &str| {
        addend_re
            .find_iter(expression)
            .map(|match_| match_.len())
            .max()
    };

    // Remove whitespace
    let equation = &space_re.replace_all(input, "");

    // Split by "==" and then "+" to reduce to lists of addends for each side of equation
    let lhs_addends: Vec<&str>;
    let rhs_addends: Vec<&str>;
    if let [lhs, rhs] = equation.split("==").collect::<Vec<_>>()[..] {
        // Check for invalid digit counts
        if max_digit_count(lhs) > max_digit_count(rhs) {
            return None;
        }
        lhs_addends = lhs.split('+').collect::<Vec<_>>();
        rhs_addends = rhs.split('+').collect::<Vec<_>>();
    } else {
        panic!("Invalid expression");
    }

    // Reduce these expressions to a map of variable to coefficient
    // e.g. I + BB == ILL
    //    -> 1 * I + 11 * B == 100 * I + 11 * L
    //    -> 11B - 99I - 11L = 0
    //    -> {(B: 11), (I: -99), (L: -11)}
    let mut reduced_equation = HashMap::new();
    for addend in lhs_addends.iter() {
        // Add coefficient count for each digit index of addend
        for (rank, digit) in addend.chars().rev().enumerate() {
            *reduced_equation.entry(digit).or_insert(0_i64) += 10_i64.pow(rank as u32);
        }
    }
    for addend in rhs_addends.iter() {
        // Subtract coefficient count for each digit index of addend
        // i.e. if the rhs is subtracted from each side to leave 0 on the rhs
        for (rank, digit) in addend.chars().rev().enumerate() {
            *reduced_equation.entry(digit).or_insert(0_i64) -= 10_i64.pow(rank as u32);
        }
    }
    // Extract into vectors of variable and coefficient with matching index
    let (variables, coefficients): (Vec<char>, Vec<i64>) = reduced_equation
        .into_iter()
        .unzip::<char, i64, Vec<char>, Vec<i64>>(
    );

    // Find variables that cannot be zero as they appear as a leading digit for a number
    let non_zero_variables: HashSet<char> = addend_re
        .find_iter(equation)
        .map(|addend| addend.as_str().chars().next().unwrap())
        .collect();

    // Iterate all permutations of variable values to find a set that satisfy the
    // equation, i.e. multiply with their coefficients and sum to zero
    'outer: for permutation in (0..=9).permutations(variables.len()) {
        match permutation
            .iter()
            .zip(&coefficients)
            .map(|(&variable, &coefficient)| variable * coefficient)
            .sum::<i64>()
            == 0
        {
            true => {
                // Create a map of variable to value
                let variable_map = variables
                    .iter()
                    .zip(permutation.iter())
                    .map(|(&variable, &value)| (variable, value as u8))
                    .collect::<HashMap<_, _>>();

                // Ensure this map does not have leading digits with a value of zero
                for (&variable, &value) in variable_map.iter() {
                    if value == 0 && non_zero_variables.contains(&variable) {
                        continue 'outer;
                    }
                }

                // Success
                return Some(variable_map);
            }
            false => {
                continue;
            }
        }
    }

    None
}
