use std::collections::HashMap;
use ndarray::Array2;

pub fn gen_bags_of_words(text_list: Vec<String>, count_duplicates: bool) -> Vec<Array2<f64>> {
    let mut dictionary: HashMap<String, usize> = HashMap::new();
    
    //populates dictionary with all the words in the dataset
    let mut counter: usize = 0;
    for text in text_list.iter() {
        for word in text.split_whitespace() {
            if !dictionary.contains_key(word) {
                dictionary.insert(word.to_owned(), counter);
                counter += 1;
            }
        }
    }

    let unique_word_amount = dictionary.len();
    let mut bags_of_words: Vec<Array2<f64>> = Vec::new();
    
    //creating bags of words
    for i in 0..text_list.len() {
        bags_of_words.push(Array2::from_elem((1, unique_word_amount), 0.0));
        
        for word in text_list[i].split_whitespace() {
            let bow_word_index = *dictionary.get(word.into()).unwrap();

            if count_duplicates {
                bags_of_words[i][(0, bow_word_index)] += 1.0;
            }
            else {
                bags_of_words[i][(0, bow_word_index)] = 1.0;
            }
            
        }
    }

    bags_of_words
}