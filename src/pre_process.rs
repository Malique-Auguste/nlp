mod bow;
mod clean;
mod porter;

use ndarray::Array2;
use std::collections::HashMap;

use ml::neural_net::training_helpers::TData;

pub fn pre_process_reviews(text_list: Vec<(String, u8)>) -> [TData; 2] {
    println!("Running pre-processing (reviews).");

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

    let mut bow_gen = bow::BowGenerator::new();
    bow_gen.expand_dictionary(&text, false);

    let mut training_input: Vec<Array2<f64>> = bow_gen.gen_bow(&text, false);
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


pub fn pre_process_auto_complete(text_list: Vec<(String, u8)>, output_length: usize) -> [TData; 2] {
    println!("Running pre-processing (auto complete).");

    //clean text
    let cleaned_text_list: Vec<String> = text_list.into_iter().map(|(text, _)| clean::space_out_non_alphanumeric(text.to_lowercase())).collect();  
    let mut dictionary: HashMap<String, usize> = HashMap::new();
    
    //populates dictionary with all the words in the dataset
    let mut counter: usize = 0;
    for text in cleaned_text_list.iter() {
        for word in text.split_whitespace() {
            if !dictionary.contains_key(word) {
                dictionary.insert(word.to_owned(), counter);
                counter += 1;
            }
        }
    }

    println!("\t1) Cleaned raw text ({} unique words).", counter);

    let sub_strings_and_output: Vec<(String, String)> = cleaned_text_list.into_iter().fold(Vec::new(), |mut acc, text| {
        let temp: Vec<&str> = text.split_whitespace().collect();
        let mut i = 0;

        while i + output_length + 1 < temp.len() {
            acc.push((temp[i..(i + output_length)].join(" "), temp[i + output_length + 1].into()));
            i += 1;
        }

        acc
    });

    println!("\t2) Generated sub strings from text ({} sub strings of length {}).", sub_strings_and_output.len(), output_length);
    let (text, output): (Vec<String>, Vec<String>) = sub_strings_and_output.into_iter().unzip();

    let mut bow_gen = bow::BowGenerator::new();
    bow_gen.expand_dictionary(&text, false);
    bow_gen.expand_dictionary(&output, false);

    let mut training_input: Vec<Array2<f64>> = bow_gen.gen_bow(&text, false);
    let mut training_output: Vec<Array2<f64>> = bow_gen.gen_bow(&output, false);
    
    println!("\t3) Generated ml input and output data (bow contains: {} unique words).", bow_gen.unique_word_amount());


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