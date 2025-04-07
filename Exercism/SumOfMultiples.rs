pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {

    let mut energyPoints = 0;

    for i in 1..limit {
        for &j in factors {
            if j != 0 && i % j == 0  {
                energyPoints += i;
                break;
            }
        }
    }
    return energyPoints;
}