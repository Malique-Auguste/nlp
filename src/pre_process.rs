mod bow;
mod clean;
mod porter;

use ndarray::Array2;
use std::collections::HashMap;

use ml::neural_net::training_helpers::TData;

pub fn pre_process(text_list: Vec<(String, u8)>) -> [TData; 2] {
    println!("Running pre-processing.");

    //clean text
    let cleaned_text_list: Vec<(String, u8)> = text_list.into_iter().map(|(text, rating)| (clean::remove_non_alphanumeric(text.to_lowercase()), rating)).collect();  
    let mut dictionary: HashMap<String, usize> = HashMap::new();
    
    //populates dictionary with all the words in the dataset
    let mut counter: usize = 0;
    for (text, _) in cleaned_text_list.iter() {
        for word in text.split_whitespace() {
            if !dictionary.contains_key(word) {
                dictionary.insert(word.to_owned(), counter);
                counter += 1;
            }
        }
    }

    println!("\t1) Cleaned raw text ({} unique words).", counter);

    //stemmed text
    let stemmed_text_list: Vec<(String, u8)> = cleaned_text_list.into_iter().map(|(text, rating)| 
        (text.split(" ").map(|word| porter::stem(word.into())).fold(String::new(), |mut a, b| {
            a.push(' ');
            a.push_str(&b);
            a
        }), rating)
    ).collect();

    println!("\t2) Stemmed cleaned text.");

    //and rating
    let (text, ratings): (Vec<String>, Vec<u8>)  = stemmed_text_list.into_iter().unzip();

    let mut training_input: Vec<Array2<f64>> = bow::gen_bags_of_words(text, false, false);
    let mut training_output: Vec<Array2<f64>> = ratings.into_iter().map(|r| Array2::from_elem((1,1), r as f64/10.0)).collect();
    println!("\t3) Generated ml input and output data (bow contains: {} unique words).", training_input[0].shape()[1]);


    //seperates out 1/5 of data to make testing data
    let testing_data_start = training_input.len() * 4 / 5;

    let testing_input: Vec<Array2<f64>> = training_input.drain(testing_data_start..).collect();
    let testing_output: Vec<Array2<f64>> = training_output.drain(testing_data_start..).collect();

    let train_len = training_input.len();
    let test_len = testing_input.len();


    let training_data = TData::new(training_input, training_output).unwrap();
    let testing_data = TData::new(testing_input, testing_output).unwrap();


    println!("\t4) Seperated training from testing data (train len: {}, test len: {}).", train_len, test_len);
    println!("Pre-processing complete.");

    [training_data, testing_data]
}