impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    let mut char_index: HashMap<char, usize> = HashMap::new();
    let mut max=0;
    let mut left =0;
    for (right, c) in s.chars().enumerate(){
       if let Some(&prev) = char_index.get(&c){
        left = left.max(prev+1);
       }
       char_index.insert(c,right);
       max= max.max(right +1 - left)
    }
max as i32
    }
}
