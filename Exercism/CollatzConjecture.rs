pub fn collatz(n: u64) -> Option<u64> {

    if n == 0 {
        return None;
    }
    
    let mut number = n;
    let mut steps_to_one = 0;

    while number > 1{
        
        if number % 2 == 0{
            number /= 2;
            
        } else {   
            number = number * 3 + 1;
        }
        steps_to_one += 1;
    }
    Some(steps_to_one)
}
