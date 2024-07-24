use std::collections::HashMap;
use ndarray::Array2;

pub struct BowGenerator {
    dictionary: HashMap<String, usize>
}

impl BowGenerator {
    pub fn new() -> BowGenerator {
        BowGenerator {
            dictionary: HashMap::new()
        }
    }

    pub fn gen_bags_of_words(&mut self, text_list: Vec<String>, count_duplicates: bool, print_dict: bool) -> Vec<Array2<f64>> {        
        //populates dictionary with all the words in the dataset
        let mut counter: usize = 0;
        for text in text_list.iter() {
            for word in text.split_whitespace() {
                if !self.dictionary.contains_key(word) {
                    self.dictionary.insert(word.to_owned(), counter);
                    counter += 1;
                }
            }
        }
    
        let unique_word_amount = self.dictionary.len();
        let mut bags_of_words: Vec<Array2<f64>> = Vec::new();
        
        //creating bags of words
        for i in 0..text_list.len() {
            bags_of_words.push(Array2::from_elem((1, unique_word_amount), 0.0));
            
            for word in text_list[i].split_whitespace() {
                let bow_word_index = *self.dictionary.get(word.into()).unwrap();
    
                if count_duplicates {
                    bags_of_words[i][(0, bow_word_index)] += 1.0;
                }
                else {
                    bags_of_words[i][(0, bow_word_index)] = 1.0;
                }
                
            }
        }
    
        if print_dict {
            let mut dict_inner = self.dictionary.iter().collect::<Vec<(&String, &usize)>>();
            dict_inner.sort_by(|a, b| a.1.cmp(b.1));
    
            println!("BOW DICTIONARY: {:?}", dict_inner);
        }
        
    
        bags_of_words
    }
}