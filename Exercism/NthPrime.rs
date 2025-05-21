pub fn nth(n: u32) -> u32 {
    let mut primeCount = 0;
    let mut value = 1;

    fn is_prime(n: u32) -> bool {
        if n < 2 {
            return false;
        }
        
        if n == 2 {
            return true;
        }
        
        if n % 2 == 0 {
            return false;
        }   

        for i in (3..).step_by(2).take_while(|i| i * i <= n){
            if n % i == 0 {
                return false;
            }
        }
        true
    }
    
    while primeCount <= n {
        value += 1;
        
        if is_prime(value) {
            primeCount += 1;
        }
    }
    value
}