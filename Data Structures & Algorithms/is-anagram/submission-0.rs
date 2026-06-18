impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
      if s.len()!=t.len(){
        return false;
      }
      let mut counts = [0i32; 26];
    for c in s.bytes() {          // c: u8
    counts[(c - b'a') as usize] += 1;   // u8 - u8, clean
   }
 for c in t.bytes() {
    counts[(c - b'a') as usize] -= 1;
}

counts.iter().all(|count| *count == 0)
    }
}
