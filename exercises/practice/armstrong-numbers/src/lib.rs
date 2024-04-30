pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let len = num.to_string().len() as u32;
    let mut sum: u32 = 0;

    for d in num.to_string().chars().map(|c| c.to_digit(10).unwrap()) {
        sum = match d.checked_pow(len).and_then(|v| v.checked_add(sum)) {
            None => return false,
            Some(v) => v,
        }
    }

    num == sum
}
