
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
     let mut low: i64 = 1;
     let mut high:i64 = *piles.iter().max().unwrap() as i64;
     while low < high {
        let mid = low + ( high -low) /2;
        let hours : i64 = piles.iter().map(|&p|(p as i64 + mid -1)/mid).sum();
        if hours <= h as i64 {
            high = mid;
        }
        else {
            low = mid + 1;
        }
     }
     low as i32
    }
}
