pub fn is_armstrong_number(num: u32) -> bool {
    let string_number = num.to_string();
    let string_length_int = string_number.len() as u32;
    let mut running_total = 0;

    for char in string_number.chars() {
        let digit = char.to_digit(10).unwrap();
        running_total += digit.pow(string_length_int);
    }

    num == running_total
}