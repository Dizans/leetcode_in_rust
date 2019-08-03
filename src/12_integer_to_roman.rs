struct Solution{
    
}

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let carrier = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
        let roman = [
            "M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I",
        ];

        let mut num = num;
        let mut result = String::from("");
        let transform: Vec<_> = carrier.iter().zip(roman.iter()).collect();
        while num != 0 {
            for (x, v) in transform.iter() {
                while num / **x != 0 {
                    num -= **x;
                    result.push_str(**v);
                }
            }

        }    result
    }
}