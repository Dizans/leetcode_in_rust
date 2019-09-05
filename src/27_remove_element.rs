struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let len = nums.len();
        if len==0{
            return 0;
        }
        let mut curr_pos=0;
        for i in 0..len{
            let curr_val = *nums.get(i).unwrap();
            if curr_val != val{
                nums[curr_pos] = curr_val;
                curr_pos +=1;
            }
        }
        curr_pos as i32
    }
}