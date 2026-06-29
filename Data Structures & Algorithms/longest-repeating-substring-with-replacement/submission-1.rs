impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let k = k as usize;
        let mut count = HashMap::new();
        let mut res = 0;
        let mut l = 0;
        let mut maxf = 0;

        for r in 0..s.len() {
            let e = count.entry(s[r]).or_insert(0usize);
            *e += 1;
            maxf = maxf.max(*e);

            while (r - l + 1) - maxf > k {
                let e = count.get_mut(&s[l]).unwrap();
                *e -= 1;
                l += 1;
            }
            res = res.max(r - l + 1);
        }

        res as i32
    }
}