use std::{fs};


use ml::neural_net::{NeuralNet, net_shape::NetShape, net_layer::*, act_func::ActFunc, training_helpers::*};

mod pre_process;


fn main() {
    let data_number: usize = 100;

    

    //read raw text data
    let text_list: Vec<(String, u8)> = load_text_data(data_number);
    
    println!("Loading raw text data ({} files attempted, {} read).\n", data_number, text_list.len());

    let [training_data, testing_data] = pre_process::pre_process(text_list);



    //AI
    println!("\nRunning ai training.");
    let shape = NetShape::new(vec![NetLayerType::DenseLayer { input_node_num: 3919, output_node_num: 391, act_func: ActFunc::Sigmoid },
                                                            NetLayerType::DenseLayer { input_node_num: 391, output_node_num: 4, act_func: ActFunc::Sigmoid },
                                                            NetLayerType::DenseLayer { input_node_num: 4, output_node_num: 1, act_func: ActFunc::Sigmoid }]).unwrap();

    println!("\t1) Creating ai.");
    
    let mut nn = NeuralNet::new(shape, 0).unwrap();
    let tsettings = TSettings::new(100, 0.005, false, 20).unwrap();
    println!("\t2) NN created and ready to train.");
    


    nn.train(training_data, Some(testing_data), &tsettings).unwrap();
    println!("\t3) NN trained.");
    println!("AI training complete.");



}

fn load_text_data(data_num: usize) -> Vec<(String, u8)> {
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