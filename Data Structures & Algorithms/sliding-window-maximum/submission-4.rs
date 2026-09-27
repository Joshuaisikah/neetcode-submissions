impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let n = nums.len();
        let mut output = Vec::with_capacity(n - k + 1);
        let mut q = VecDeque::new();
        let mut l = 0;

        for r in 0..n {
            while let Some(&back) = q.back() {
                if nums[back] < nums[r] {
                    q.pop_back();
                } else {
                    break;
                }
            }
            q.push_back(r);

            if l > *q.front().unwrap() {
                q.pop_front();
            }

            if r + 1 >= k {
                output.push(nums[*q.front().unwrap()]);
                l += 1;
            }
        }

        output
    }
}