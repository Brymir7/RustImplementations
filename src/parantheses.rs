pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => {
                stack.push(c);
            }
            ')' | ']' | '}' => {
                let Some(last_opening_bracket) = stack.pop() else {
                    return false;
                };
                if !matches!((last_opening_bracket, c), ('(', ')') | ('[', ']') | ('{', '}')) {
                    return false;
                }
            }
            _ => {
                continue;
            }
        }
    }
    return stack.len() == 0;
}

pub fn test_parantheses_checker() {
    assert_eq!(is_valid("()".into()), true);
    assert_eq!(is_valid("()[]{}".into()), true);
    assert_eq!(is_valid("(]".into()), false);
    assert_eq!(is_valid("([]{})".into()), true);
}
