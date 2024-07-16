use std::fs;
mod porter;

fn main() {
    let text_list = read_text_data(20);
    println!("{:#?}", text_list);
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