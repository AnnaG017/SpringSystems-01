fn most_frequent_word(text: &str) -> (String, usize) {
    let word: Vec<&str> = text.split_whitespace().collect(); //split the text and stores it in a vector
    let mut max_word = ""; // store the most frequent word found so far
    let mut max_count = 0;//starts at zero

    for i in 0..word.len(){
        let mut count = 0;// Count how many times the word[i] appears
        for j in 0..word.len(){
            //If the words match, increase the count
            if word[i]==word[j]{
                count += 1;
            }
        // If this word appears more times than the current max
        // updates the maximum count and stores the word 
        if count > max_count{
            max_count = count; 
            max_word = word[i];
        }

        }
    }
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
  