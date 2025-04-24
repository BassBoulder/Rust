pub fn square_of_sum(n: u32) -> u32 {
    let mut square_sum = 0;
    let mut i = 1;

    for i in 1..n+1 {
        square_sum += i;

    }
    square_sum * square_sum
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut sum_squares = 0;
    let mut i = 1;

    for i in 1..n+1 {
        sum_squares += i * i;

    }
    sum_squares
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
