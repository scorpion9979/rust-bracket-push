fn brackets_match(open_bracket: char, close_bracket: char) -> bool {
    match (open_bracket, close_bracket) {
        ('[', ']') | ('(', ')') | ('{', '}') => true,
        (_, _) => false,
    }
}

fn is_open_bracket(ch: char) -> bool {
    ch == '[' || ch == '(' || ch == '{'
}

fn is_close_bracket(ch: char) -> bool {
    ch == ']' || ch == ')' || ch == '}'
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_brackets_match() {
        assert!(brackets_match('[', ']'));
        assert!(brackets_match('(', ')'));
        assert!(brackets_match('{', '}'));
        assert!(!brackets_match('{', ')'));
        assert!(!brackets_match('[', '}'));
        assert!(!brackets_match(')', ']'));
    }
}

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = String::from("");
    for ch in string.chars() {
        if is_open_bracket(ch) {
            stack.push(ch);
        } else if is_close_bracket(ch) {
            if let Some(last_open_bracket) = stack.pop() {
                if brackets_match(last_open_bracket, ch) {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }
    stack.is_empty()
}
