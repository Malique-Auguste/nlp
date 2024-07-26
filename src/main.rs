


use ml::neural_net::{NeuralNet, net_shape::NetShape, net_layer::*, act_func::ActFunc, training_helpers::*};

mod pre_process;
mod data_handler;


fn main() {
    
    let data_number: usize = 400;

    
    
    //read raw text data
    let text_list: Vec<(String, u8)> = data_handler::load_text_data(data_number);
    
    println!("Loading raw text data ({} files attempted, {} read).\n", data_number, text_list.len());

    let [training_data, testing_data] = pre_process::pre_process_reviews(text_list);



    //AI
    println!("\nRunning ai training.");
    
    let shape = NetShape::new(vec![NetLayerType::DenseLayer { input_node_num: 9326, output_node_num: 311, act_func: ActFunc::Sigmoid },
                                                            NetLayerType::DenseLayer { input_node_num: 311, output_node_num: 9, act_func: ActFunc::Sigmoid },
                                                            NetLayerType::DenseLayer { input_node_num: 9, output_node_num: 1, act_func: ActFunc::Sigmoid }]).unwrap();
    
    println!("\t1) Creating ai.");
    
    let mut nn = NeuralNet::new(shape, 0).unwrap();
    

    /*
    println!("\t1) Loading ai.");
    let mut nn = NeuralNet::load("saved_nn/alpha/1/nn_min_train_err").unwrap();
    */
    
    let tsettings = TSettings::new(100, 0.005, true, 20, Some("saved_nn/hidden1/1/nn_min_train_err".into()), Some("saved_nn/hidden1/1/nn_min_test_err".into())).unwrap();
    println!("\t2) NN ready to train.");
    


    nn.train(training_data, Some(testing_data), &tsettings).unwrap();
    println!("\t3) NN trained.");
    println!("AI training complete.");

    

}