//splits lengths of text into fragments of specific length with following word as output

pub fn gen_input_output(input: Vec<String>, output_length: usize) -> Vec<(String, String)> {
    input.into_iter().fold(Vec::new(), |mut acc, text| {
        let temp: Vec<&str> = text.split_whitespace().collect();
        let mut i = 0;

        while i + output_length + 1 < temp.len() {
            acc.push((temp[i..(i + output_length)].join(" "), temp[i + output_length + 1].into()));
            i += 1;
        }

        acc
    })
}