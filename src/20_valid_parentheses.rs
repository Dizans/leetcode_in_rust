use std::collections::{HashMap, HashSet};
impl Solution {

    pub fn is_valid(s: String) -> bool {
        if s.len() == 0{
            return true;
        }
        let mut stack: Vec<char> = vec![];
        let right_parentheses: HashSet<char> =
            ['}', ']', ')'].iter().cloned().collect();

        let mut pair = HashMap::new();
        pair.insert('}','{');
        pair.insert(']', '[');
        pair.insert(')', '(');

        for c in s.chars(){
            if right_parentheses.contains(&c){
                if stack.len() ==0{
                    return false;
                }
                if *stack.last().unwrap()!= *pair.get(&c).unwrap(){
                    return false;
                }
                stack.pop();
            }else{
                stack.push(c);
            }
        }
        return stack.len() ==0;

    }
}