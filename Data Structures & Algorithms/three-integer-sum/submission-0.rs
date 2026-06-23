impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut res = Vec::new();
    let n = nums.len();
    for i in 0..n{
        let a = nums[i];
        if a > 0 {
            break;
        }
        if i > 0 && a == nums[i -1]{
            continue;
        }
        let (mut l,mut r) = ( i+ 1,n-1);
        while l <r {
            let sum = a+nums[l]+ nums[r];
            if sum> 0 {
                r -=1;
            }else if sum  <0 {
                l +=1;
            }else {
                res.push(vec![a,nums[l],nums[r]]);
                l +=1;
                r -=1;
                while l < r && nums[l] == nums[l -1]{
                    l +=1;
                }
        }
        }
    }
    res
    }
}
