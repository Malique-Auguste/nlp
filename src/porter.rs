pub fn stem(mut word: String) -> String {
    let (m, is_vowel_list) = calc_m(&word);
    println!("m for {}: {}", word, m);

    //performs step 1a
    let mut step_successful = {
        let step_1a_replacement_pairs = vec![("sses", "ss", 1), ("ies", "i", 1), ("ss", "ss", 0), ("s", "", 0)];
        find_and_replace(&mut word, m, step_1a_replacement_pairs)
    };

    if step_successful {
        return word
    }

    //perfomrs step 1b
    step_successful = {
        let step_1b_replacement_pairs = vec![("eed", "ee", 1), ("ed", "", 2), ("ing", "", 2)];
        find_and_replace(&mut word, m, step_1b_replacement_pairs)
    };

    if step_successful {
        step_successful = {
            let step_1b2_replacement_pairs = vec![("at", "ate", 0), ("bl", "ble", 0), ("iz", "ize", 0)];
            find_and_replace(&mut word, m, step_1b2_replacement_pairs)
        };

        if step_successful {
            return word
        }
        else {
            step_successful = {
                if !word.ends_with(['l', 's', 'z']) {
                    let letters = word.chars();
                    if letters.last() == letters.last() {
                        word.pop();
                        true
                    }
                    else {
                        false
                    }
                }
                else {
                    let (temp_m, temp_is_vowel_list) = calc_m(&word);
                    false
                }
            }
        }
    }
    
    word
}


fn calc_m(word: &str) -> (u8, Vec<bool>) {
    if word.len() <= 2 {
        return 0
    }

    //list stating if a character in a word is a vowel or not
    let mut is_vowel_list = Vec::new();
    for letter in word.chars() {
        is_vowel_list.push(is_vowel(letter, is_vowel_list.last()))
    }

    //simplifies vowels/constanants following each other 
    let mut i: usize = 0;
    while i < is_vowel_list.len() - 1 {
        if is_vowel_list[i] == is_vowel_list[i + 1] {
            is_vowel_list.remove(i + 1);
        }
        else {
            i += 1;
        }
    }

    //removes first character if it starts with a constanant
    if is_vowel_list[0] == false {
        is_vowel_list.remove(0);
    }

    //number of VC pairs
    //i.e number of times vowels are followed by consonants
    let m: u8 = (is_vowel_list.len() / 2) as u8;

    (m, is_vowel_list)
}

//determiens if a letter is a vowel
//vowels are a,e,i,o,u or y if it follows a constanant
//such as in Philly but not in yes
fn is_vowel(letter: char, previous_letter_is_vowel: Option<&bool>) -> bool {
    match letter {
        'a' | 'e'| 'i' | 'o' | 'u' => true,
        'y' => match previous_letter_is_vowel {
            Some(true) => false,
            Some(false) => true,
            None => false 
        }
        _ => false
    }
}

fn find_and_replace(word: &mut String, m: u8, replacement_pairs: Vec<(&str, &str, u8)>) -> bool {
    for pair in replacement_pairs {
        if word.ends_with(pair.0) {
            if m >= pair.2 {
                let index = word.find(pair.0).unwrap();
                word.replace_range(index.., pair.1);
                return true
            }
        }
    }

    false
}