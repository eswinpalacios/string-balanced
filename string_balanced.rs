use std::collections::HashMap;

fn is_string_balanced(list_characters: &str) -> bool {    
    let open: HashMap<&char, char> = [ (&'{', '{'), (&'[', '['), (&'(', '(') ].iter().cloned().collect();
    let close: HashMap<&char, char> = [ (&'}', '{'), (&']', '['), (&')', '(') ].iter().cloned().collect();
    let mut stack:Vec<char> = Vec::new();    

    for character in list_characters.chars() {
        if open.contains_key(&character) {
            stack.push(character)
        } else if close.contains_key(&character) {
            let symbol = close.get(&character);
            if stack.len() == 0 {
                return false
            } else if Some(&stack[stack.len() - 1 ]) != symbol {
                return false
            } else {
                stack.pop();
            }
        }
    }

    return if stack.len() == 0 { true } else { false }
}

fn main() {
    println!("{}", is_string_balanced("(a[0]+b[2c[6]]) {24+53})"));
    println!("{}", is_string_balanced("f(e(d))[()]{}([])'"));
    println!("{}", is_string_balanced("((b)"));
    println!("{}", is_string_balanced("(c]"));
    println!("{}", is_string_balanced("{(a[])"));
    println!("{}", is_string_balanced("([)]"));
    println!("{}", is_string_balanced(")("));
    println!("{}", is_string_balanced(""));
    println!("{}", is_string_balanced("([)]"));
}
