pub fn raindrops(n: u32) -> String {

    let mut raindrops_outcome = String::new();

    // 3 = No
    if n % 3 == 0 {
        raindrops_outcome += "Pling";
    }
    
    // 5 = Yes
    if n % 5 == 0 {
        raindrops_outcome += "Plang";
    }

    // 7 = Yes
    if n % 7 == 0 {
        raindrops_outcome += "Plong";
    }
    
    // 7 & 3 & 5 = No
    if n % 3 != 0 && n % 5 != 0 && n % 7 != 0  {
        raindrops_outcome = n.to_string();
    }
    
return raindrops_outcome;
}