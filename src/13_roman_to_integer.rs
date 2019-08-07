use std::collections::HashMap;

pub fn gredy_match(word: &str, map: &HashMap<&str, i32>) -> i32{
    if word.len() ==0{
        return 0;
    }
    if word.len() ==1{
        return *map.get(word).unwrap();
    }
    
    let count = map.get(&word[0..2]);
    match count {
        None => return map.get(&word[0..1]).unwrap() + gredy_match(&word[1..], &map),
        Some(k) =>return k +  gredy_match(&word[2..], &map)
    }
}

impl Solution {

pub fn roman_to_int(s: String) -> i32 {
    let mut map: HashMap<&str, i32> = HashMap::new();

    map.insert("M", 1000);
    map.insert("CM", 900);
    map.insert("D", 500);
    map.insert("CD", 400);
    map.insert("C", 100);
    map.insert("XC", 90);
    map.insert("L", 50);
    map.insert("XL", 40);
    map.insert("X", 10);
    map.insert("IX", 9);
    map.insert("V", 5);
    map.insert("IV", 4);
    map.insert("I", 1);

    gredy_match(&s[..], &map)
 }

}