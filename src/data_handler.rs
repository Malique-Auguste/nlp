use std::fs;

pub fn load_text_data(data_num: usize) -> Vec<(String, u8)> {
    let mut text_list: Vec<(String, u8)> = Vec::new();
    text_list.append(&mut get_folder_content("aclImdb/train/pos", data_num * 4 / 10));
    text_list.append(&mut get_folder_content("aclImdb/train/neg", data_num * 4 / 10));
    text_list.append(&mut get_folder_content("aclImdb/test/pos", data_num * 1 / 10));
    text_list.append(&mut get_folder_content("aclImdb/test/neg", data_num * 1 / 10));

    text_list
}

fn get_folder_content(path: &str, amount_of_content: usize) -> Vec<(String, u8)> {
    let mut text_list: Vec<(String, u8)> = Vec::new();

    //positive ratings
    let mut counter = 0;
    for entry in fs::read_dir(path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            counter += 1;

            let file_name: String = path.file_stem().unwrap().to_str().unwrap().into();
            //println!("{}", file_name);
            let rating: u8 = (file_name.split('_').last().unwrap()).parse().unwrap();

            let file_content = fs::read_to_string(path.clone()).unwrap();

            if counter > amount_of_content {
                break
            }
            else {
                text_list.push((file_content, rating));
            }
        }
    }

    text_list
}