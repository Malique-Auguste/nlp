use std::collections::HashMap;
use ndarray::Array2;

//BOW = Bag of Words
pub struct BowGenerator {
    dictionary: HashMap<String, usize>
}

impl BowGenerator {
    pub fn new() -> BowGenerator {
        BowGenerator {
            dictionary: HashMap::new()
        }
    }

    pub fn expand_dictionary(&mut self, text_list: &Vec<String>, print_dict: bool) {        
        //populates dictionary with all the words in the dataset
        let mut counter: usize = self.dictionary.len();
        for text in text_list.iter() {
            for word in text.split_whitespace() {
                if !self.dictionary.contains_key(word) {
                    self.dictionary.insert(word.to_owned(), counter);
                    counter += 1;
                }
            }
        }

        if print_dict {
            let mut dict_inner = self.dictionary.iter().collect::<Vec<(&String, &usize)>>();
            dict_inner.sort_by(|a, b| a.1.cmp(b.1));
    
            println!("BOW DICTIONARY: {:?}", dict_inner);
        }
    }

    pub fn gen_bow (&mut self, text_list: &Vec<String>, count_duplicates: bool) -> Vec<Array2<f64>> {
    
        let unique_word_amount = self.unique_word_amount();
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
    
        bags_of_words
    }

    pub fn get_word_index(&self, word: &str) -> Option<&usize> {
        self.dictionary.get(word)
    }

    pub fn get_words_index_as_arr(&self, list: Vec<String>) -> Result<Vec<Array2<f64>>, String> {
        let mut output: Vec<Array2<f64>> = Vec::new();
        for word in list {
            match self.dictionary.get(&word) {
                Some(num) => output.push(Array2::from_elem((1,1), *num as f64)),
                None => return Err(format!("'{}', not found in dicitonary.", word))
            }
        }

        Ok(output)
    }

    pub fn unique_word_amount(&self) -> usize {
        self.dictionary.len()
    }
}