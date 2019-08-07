
struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut result = String::from("");
        if strs.len() == 0 {
            return result;
        }
        let mut min_len = std::usize::MAX;
        for x in strs.iter() {
            if x.len() < min_len {
                min_len = x.len();
            }
        }
        let first_string: &String = strs.get(0).unwrap();
        for ith in 0..min_len {
            let c: char = first_string.chars().nth(ith).unwrap();
            for s in strs.iter() {
                let w = s.chars().nth(ith).unwrap();
                if w != c {
                    return result;
                }
            }
            result.push_str(&c.to_string()[..]);
        }
        result
    }
}