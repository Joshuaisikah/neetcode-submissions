impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
     let mut map:HashMap<i32, usize> = HashMap::new();
     for( i,&num) in nums.iter().enumerate(){
        let diff = target - num;
        if let Some(&j) =map.get(&diff){
            if j< i{
                return vec![j as i32, i as i32];
            }else {
                return vec![i as i32, j as i32];
            }
        }
        map. insert(num ,i);
     }
     unreachable!()
    }
}
