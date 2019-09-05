struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let len = nums.len();
        if len==0{
            0
        }else if len==1{
            1
        }else{
            let mut pos = 1;
            let mut no_dup_pos = 0;
            let mut n = 1;

            while pos < nums.len(){
                let curr = nums.get(no_dup_pos).unwrap();
                let last = nums.get(pos).unwrap();
                if curr == last{
                    pos +=1;
                }else{
                    no_dup_pos+=1;
                    nums[no_dup_pos] = *last;
                    pos +=1;
                    n+=1;
                }
            }
            for i in no_dup_pos..(len-1){
                nums.pop();
            }
            n
        }
    }
}