// -----------------------------------------------------------------------------
pub fn is_valid(s: String) -> bool {
    // the order of parentheses 'type' matters, so use a LIFO stack
    let mut stack = Vec::new();

    for i in s.chars() {
        match i {
            '{' => stack.push('}'),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            // when a closing parenthesis is encountered while iterating through s.chars(),
            //  check if the last stack element (.pop()) matches the type required by the order.
            //  if not, this means the sequence of closing parentheses is out of order and NOT valid
            '}' | ')' | ']' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}
