impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
    let mut left= 0;
    let mut right = heights.len() -1;
    let mut max =0;
    while left < right{
         let   area = heights[left].min(heights[right]) * (right -left) as i32;
         max = max.max(area);
         if heights[left] <= heights[right]{
            left +=1;
         }else{
            right -=1;
         }
    }



    max
    }
}
