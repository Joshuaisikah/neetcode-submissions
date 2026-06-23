impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
     let mut left =0;
     let mut right = numbers.len().saturating_sub(1);
     while left < right{
        let current_sum =numbers[left] +numbers[right];
        match current_sum.cmp(&target){
            Ordering::Equal =>{ return vec![(left +1 ) as i32 ,(right +1) as i32];
            }
        
        Ordering::Greater=>{
            right -=1;
        }
        Ordering::Less=>{
            left +=1;
     }
            }
     }
  vec![]
    }
}
