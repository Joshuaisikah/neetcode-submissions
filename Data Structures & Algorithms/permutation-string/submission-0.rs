impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1= s1.as_bytes();
       let s2 = s2.as_bytes(); 
       if s1.len() > s2.len() {
        return false;
       }
     let mut s1_count = [0i32; 26];
     let mut s2_count = [0i32; 26];
      for i in 0..s1.len() {
            s1_count[(s1[i] - b'a') as usize] += 1;
            s2_count[(s2[i] - b'a') as usize] += 1;
        }
         let mut matches = 0;
        for i in 0..26 {
            if s1_count[i] == s2_count[i] {
                matches += 1;
            }
        }
         let mut l = 0;
        for r in s1.len()..s2.len() {
            if matches == 26 {
                return true;
            }
            let idx = (s2[r] - b'a') as usize;
            s2_count[idx] += 1;
            if s1_count[idx] == s2_count[idx] {
                matches += 1;
            } else if s1_count[idx] + 1 == s2_count[idx] {
                matches -= 1;
            }
            let idx = (s2[l] - b'a') as usize;
            s2_count[idx] -= 1;
            if s1_count[idx] == s2_count[idx] {
                matches += 1;
            } else if s1_count[idx] - 1 == s2_count[idx] {
                matches -= 1;
            }
            l += 1;
        }
        matches == 26
    
    }
}
