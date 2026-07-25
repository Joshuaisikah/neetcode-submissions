impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
            let mut ans=nums[0];

    for num in nums {
         if num<ans{
            ans = num;
         } 
    }
    ans
    }
}
