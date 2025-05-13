#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    
    if num == 0{
        return None;
    }
    
    let half_num: u64 = num / 2;
    let mut answer: u64 = 0;
    
    for i in 1..=half_num {
        if num % i == 0 {
            answer += i;
        }
    }
    
    let classification = 
        if answer < num {
            Classification::Deficient
        } else if answer == num {
            Classification::Perfect
        } else {
            Classification::Abundant
        };

    Some(classification)
}