public class Solution {
    public List<List<string>> GroupAnagrams(string[] strs) {
        var anagrams= new Dictionary<string,List<string>>();
        foreach(var str in strs){
            var charArray =str.ToArray();
            Array.Sort(charArray);
            var sortedStr = new string(charArray);
            if(!anagrams.TryGetValue(sortedStr , out var group)){
                group = new List<string>();
                anagrams[sortedStr] = group;
            }
            group.Add(str);     
              }
     return anagrams.Values.Select(group => (List<string>)group).ToList();
    }
}
