impl Solution {
    pub fn is_palindrome(s: String) -> bool {
   let mut cleaned:Vec<char> = s.chars().filter(|c|c.is_alphanumeric())
                        .map(|c|c.to_ascii_lowercase())
                        .collect();
                        cleaned.iter().eq(cleaned.iter().rev())
    }
}
