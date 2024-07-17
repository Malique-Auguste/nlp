use std::fs;
mod porter;

fn main() {
    let text_list = read_text_data(20);

    let text = text_list[1].0.clone();

    let cleaned_text = clean_text(text.to_lowercase());
    let split_text: Vec<&str> = cleaned_text.split(" ").collect();

    let stemmed_text: Vec<String> = split_text.iter().map(|word| porter::stem((*word).into())).collect();
    let stemmed_text = stemmed_text.join(" ");
    
    println!("Original text:\n{}\n\nCleaned text:\n{}\n\nStemmed Text:\n{}", text, cleaned_text, stemmed_text );
}

fn read_text_data(text_item_num: usize) -> Vec::<(String, u8)> {
    let mut text_list: Vec<(String, u8)> = Vec::new();

    //positive ratings
    let mut counter = 0;
    for entry in fs::read_dir("aclImdb/train/pos").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file_name: String = path.file_stem().unwrap().to_str().unwrap().into();
            let rating: u8 = (file_name.split('_').last().unwrap()).parse().unwrap();

            text_list.push((fs::read_to_string(path.clone()).unwrap(), rating));
        }

        counter += 1;
        if counter >= text_item_num / 2 {
            break
        }
    }

    //negative ratings
    counter = 0;
    for entry in fs::read_dir("aclImdb/train/neg").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let file_name: String = path.file_stem().unwrap().to_str().unwrap().into();
            let rating: u8 = (file_name.split('_').last().unwrap()).parse().unwrap();

            text_list.push((fs::read_to_string(path.clone()).unwrap(), rating));
        }

        counter += 1;
        if counter >= text_item_num / 2 {
            break
        }
    }

    text_list
}

fn clean_text(text: String) -> String {
    let mut characters: Vec<char> = text.chars().collect();

    let mut i = 0;
    while i < characters.len() {
        if characters[i].is_alphanumeric() || characters[i].is_whitespace() {
            i += 1;
        }
        else {
            characters.insert(i, ' ');
            characters.insert(i + 2, ' ');
            i += 3
        }
    }

    let output: String = characters.into_iter().collect();
    output.replace("  ", " ")
}