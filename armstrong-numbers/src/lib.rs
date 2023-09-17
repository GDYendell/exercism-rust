pub fn is_armstrong_number(num: u32) -> bool {
    let digit_count = num.to_string().len() as u32;

    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .fold(0_u64, |acc, c| {
            acc.checked_add(c.pow(digit_count) as u64).unwrap()
        })
        == num as u64
}
