

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>>
{
    let mut anagrams: std::collections::HashMap<Vec<char>, Vec<String>> = std::collections::HashMap::new();

    for s in strs
    {
        //sort current string
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort();
        /*access map at chars -> if empty add vector to be
        accessed, then push the string onto the value vector
        */
        anagrams.entry(chars).or_insert(vec![]).push(s);

    }
    return anagrams.values().cloned().collect();
}
    