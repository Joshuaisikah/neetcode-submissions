impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
     let mut output = vec![1; n];
    
  // what the above loop does is// Prefix pass: set output[i] to the product of all elements left of i.
// output[0] stays 1 (nothing to its left); each entry chains off the previous
// prefix product times the element just before it.
for i in 1..n {
    output[i] = output[i - 1] * nums[i - 1];
}

 let mut right = 1;
        for i in (0..n).rev() {
            output[i] *= right;
            right *= nums[i];
        }

     output
    }
}
