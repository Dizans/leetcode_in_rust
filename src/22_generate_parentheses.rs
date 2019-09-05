impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let permutations = get_permutations(n as u32);
        let result:Vec<String> = permutations.into_iter()
            .filter(|s| is_valid(&s)).collect();
        result
    }


}

fn get_permutations(n: u32) -> Vec<String>{
    let mut permutations = vec![];
    let length = n * 2;
    let total: u32= 2u32.pow(length);

    for i in 0..total{
        let bit_viewer = format!("{:032b}",i);
        let begin_pos = (32-length) as usize;
        let bit_viewer = &bit_viewer[begin_pos..];
        let parenthesis_viewer = String::from(bit_viewer)
            .replace("0","(")
            .replace("1", ")");
        permutations.push(parenthesis_viewer);
    }
    permutations

}

fn is_valid(s: &String) -> bool{
    let mut stack = vec![];
    // print!("{}   :", s);
    for c in s.chars(){
        if c=='('{
            stack.push(c);
        }else{
            match stack.pop(){
                None => {
                    return false;
                }
                Some(k) => {
                    if k!='('{
                        return false;
                    }
                }
            }
        }
    }
    // println!("{}", stack.len()==0);
    stack.len()==0
}