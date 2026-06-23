impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut res = Vec::new();
        let n = nums.len();

        for i in 0..n {
            let a = nums[i];

            // Early exit: no need to continue if current number is positive
            if a > 0 {
                break;
            }

            // Skip duplicates for i
            if i > 0 && a == nums[i - 1] {
                continue;
            }

            let (mut l, mut r) = (i + 1, n - 1);

            while l < r {
                let sum = a + nums[l] + nums[r];

                if sum > 0 {
                    r -= 1;
                } else if sum < 0 {
                    l += 1;
                } else {
                    res.push(vec![a, nums[l], nums[r]]);

                    // Skip duplicates for both pointers
                    l += 1;
                    r -= 1;

                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                    while l < r && nums[r] == nums[r + 1] {
                        r -= 1;
                    }
                }
            }
        }
        res
    }
}