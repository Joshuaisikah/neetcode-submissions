impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<i32>, Vec<String>> = HashMap::new();
    for s in strs{
         let mut count = vec![0;26];
         for c in s.bytes(){
            count[(c -  b'a')  as usize] +=1;
        }
        map.entry(count).or_insert_with(Vec::new).push(s);
    }
      map.into_values().collect()

    }
}
