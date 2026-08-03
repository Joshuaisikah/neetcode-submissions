impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut stones = stones;
        stones.sort_unstable();
        let mut n = stones.len();

        while n > 1 {
            let cur = stones[n - 1] - stones[n - 2];
            n -= 2;
            if cur > 0 {
                let mut l = 0usize;
                let mut r = n;
                while l < r {
                    let mid = (l + r) / 2;
                    if stones[mid] < cur {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }
                let pos = l;
                if n < stones.len() {
                    stones[n] = 0;
                } else {
                    stones.push(0);
                }
                n += 1;
                for i in (pos + 1..n).rev() {
                    stones[i] = stones[i - 1];
                }
                stones[pos] = cur;
            }
        }

        if n > 0 { stones[0] } else { 0 }
    }
}