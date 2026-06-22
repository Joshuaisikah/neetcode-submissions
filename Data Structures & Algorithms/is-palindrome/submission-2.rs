impl Solution {
    pub fn is_palindrome(s: String) -> bool {
   let mut cleaned:Vec<char> = s.chars().filter(|c|c.is_alphanumeric())
                        .map(|c|c.to_ascii_lowercase())
                        .collect();
                        let mut left =0;
                        let mut right = cleaned.len().saturating_sub(1);
                        while left< right{
                            if cleaned[left] !=  cleaned[right]{
                                return false;
                            }
                            left +=1;
                            right -=1;
                        }
                        true
    }
}
