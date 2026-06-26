impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
    if height.len() < 0 {
        return 0;
    }
     let mut left =0;
      let mut right = height.len() -1;
      let mut left_max =height[left];
      let mut right_max = height[right];
      let mut result =0;
      while left< right{
            if left_max < right_max{
                left +=1;
                left_max = left_max.max(height[left]);
                result += left_max - height[left];
            }else{
                right -=1;
                right_max = right_max.max(height[right]);
                result += right_max -height[right];
            }
      }
        result
    }
}
