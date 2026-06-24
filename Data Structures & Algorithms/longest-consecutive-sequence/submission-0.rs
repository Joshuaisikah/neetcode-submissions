impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
     let mut map: HashMap<i32,i32> = HashMap::new();
     let mut result = 0;
      for &num in &nums{
        if !map.contains_key(&num){
            let left = *map.get(&(num-1)).unwrap_or(&0);
             let right = *map.get(&(num +1)).unwrap_or(&0);
             let length =left +right +1;
              map.insert(num,length);
              map.insert(num-left,length);
              map.insert(num+right,length);
              result = result.max(length);
        }
      }
      result
    }
}
