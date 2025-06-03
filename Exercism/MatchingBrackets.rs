pub fn brackets_are_balanced(string: &str) -> bool {

    if string.is_empty(){
        return true;
    }

    let mut char_stack : Vec<char> = Vec::new();

    for char in string.chars() {
        match char {
            
            '{' | '(' | '[' => {
                 char_stack.push(char);
             }   
            
            '}' => {
                if char_stack.pop() != Some('{'){
                    return false;
                }
            }
            
            ')' => {
                if char_stack.pop() != Some('(') {
                    return false;
                }
            }
            
            ']' => {
                if char_stack.pop() != Some('[') {
                    return false;
                }
            }
            
            _ => {}
        }
    }
    
    char_stack.is_empty()
}