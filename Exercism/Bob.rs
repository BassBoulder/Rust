pub fn reply(message: &str) -> &str {

    let message = message.trim();

    let mut question = message.ends_with('?');
    let mut all_caps = message.chars().any(|c| c.is_alphabetic()) && message == message.to_uppercase();
    
    if message.is_empty(){
        "Fine. Be that way!"
    }
    else if question && all_caps {
        "Calm down, I know what I'm doing!"
    }
    else if question && !all_caps {
        "Sure."
    }
    else if !question && all_caps {
        "Whoa, chill out!"
    } else {
        "Whatever." 
    }
}