pub fn space_out_non_alphanumeric(text: String) -> String {
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

pub fn un_space_out_non_alphanumeric(text: String) -> String {
    let mut characters: Vec<char> = text.chars().collect();

    let mut i = 0;
    while i < characters.len() {
        if characters[i].is_alphanumeric() || characters[i].is_whitespace() {
            i += 1;
        }
        else {
            characters.remove(i + 1);
            characters.remove(i - 1);
        }
    }

    //output
    characters.into_iter().collect()
}

pub fn remove_non_alphanumeric(text: String) -> String {
    let mut characters: Vec<char> = text.chars().collect();

    let mut i = 0;
    while i < characters.len() {
        if characters[i].is_alphanumeric() || characters[i].is_whitespace() {
            i += 1;
        }
        else {
            characters.remove(i);
        }
    }

    let output: String = characters.into_iter().collect();
    output.replace("  ", " ")
}