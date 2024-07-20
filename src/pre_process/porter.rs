pub fn stem(mut word: String) -> String {
    if word.len() <= 3 {
        return word
    }

    let (m, is_vowel_list) = calc_m(&word);
    //println!("m and vowel list for {}: {}\t|\t{:?}", word, m, is_vowel_list);

    //if there are no vowels then this is not an english word
    if m == 0 {
        return word
    }

    //performs step 1a
    let mut step_successful = {
        let step_1a_replacement_pairs = vec![("sses", "ss", 0), ("ies", "i", 0), ("ss", "ss", 0), ("s", "", 0)];
        find_and_replace(&mut word, m, step_1a_replacement_pairs)
    };

    if step_successful {
        return word
    }

    //perfomrs step 1b
    step_successful = {
        let step_1b_replacement_pairs = vec![("eed", "ee", 1), ("ed", "", 1), ("ing", "", 1)];
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
            //step 1b2 continued
            if !word.ends_with(['l', 's', 'z']) {
                if is_double_ending(&word) {
                    word.pop();
                }
            }
            else {
                if m == 2 && is_cvc_ending(&word) {
                    word.push('e');
                }
                else {
                }
            }

            return word
        }
    }

    //step 1c
    step_successful = {
        if is_vowel_list[0..(is_vowel_list.len()-1)].contains(&true) {
            let step_1c_replacement_pairs = vec![("y", "i", 0)];
            find_and_replace(&mut word, m, step_1c_replacement_pairs)
        }
        else {
            false
        }
    };

    if step_successful {
        return word;
    }

    //step 2
    step_successful = {
        let step_2_replacement_pairs = vec![("ational", "ate", 3), ("tional", "tion", 2), ("enci", "ence", 1), 
                                                                    ("anci", "ance", 1), ("izer", "ize", 2), ("abli", "able", 1), 
                                                                    ("alli", "al", 1), ("entli", "ent", 1), ("eli", "e", 1), 
                                                                    ("ousli", "ous", 1), ("ization", "ize", 3), ("ation", "ate", 2), 
                                                                    ("ator", "ate", 2), ("alism", "al", 1), ("iveness", "ive", 3), 
                                                                    ("fulness", "ful", 2), ("ousness", "ous", 2), ("aliti", "al", 2), 
                                                                    ("iviti", "ive", 2), ("biliti", "ble", 1)];
        find_and_replace(&mut word, m, step_2_replacement_pairs)
    };

    if step_successful {
        return word
    }

    //step 3
    step_successful = {
        let step_3_replacement_pairs = vec![("icate", "ic", 2), ("ative", "", 2), ("alize", "al", 2), 
                                                                    ("iciti", "ic", 1), ("ical", "ic", 2), ("ful", "", 1),
                                                                    ("ness", "", 1)];
        find_and_replace(&mut word, m, step_3_replacement_pairs)                                                
    };

    if step_successful {
        return word
    }

    //step 4
    step_successful = {
        let step_4_replacement_pairs = vec![("al", "", 2), ("ance", "", 2), ("ence", "", 2),
                                                                    ("er", "", 2), ("ic", "", 2), ("able", "", 2), 
                                                                    ("ible", "", 2), ("ant", "", 2), ("ement", "", 3), 
                                                                    ("ment", "", 2), ("ent", "", 2), ("sion", "", 2), 
                                                                    ("tion", "", 2), ("ou", "", 1), ("ism", "", 2), 
                                                                    ("ate", "", 2), ("iti", "", 2), ("ous", "", 2), 
                                                                    ("ive", "", 2), ("ize", "", 2)];
        find_and_replace(&mut word, m, step_4_replacement_pairs)                                                
    };

    if step_successful {
        return word
    }

    //step 5a
    step_successful = {
        let step_5a_replacement_pairs = vec![("e", "", 1)];
        step_successful = find_and_replace(&mut word, m, step_5a_replacement_pairs);

        if step_successful {
            return word;
        }
        //if (m=1 and not CVC) followed by e
        else if m == 1{
            let mut word_clone = word.clone();

            if word_clone.pop().unwrap() == 'e' && !is_cvc_ending(&word_clone) {
                word.pop();
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    };

    if step_successful {
        return word
    }

    //step 5b
    if m > 1 && word.ends_with('l') && is_double_ending(&word) {
        word.pop();
    }
    
    word
}


fn calc_m(word: &str) -> (u8, Vec<bool>) {
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

    //number of VC pairs
    //i.e number of times vowels are followed by consonants
    let m: u8 = {
        //ignores first character if it starts with a constanant
        if is_vowel_list[0] == false {
            ((is_vowel_list.len() - 1) / 2) as u8
        }
        else {
            (is_vowel_list.len() / 2) as u8
        }
    };

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
            if m > pair.2 {
                let index = word.find(pair.0).unwrap();
                word.replace_range(index.., pair.1);
                return true
            }
        }
    }

    false
}

fn is_double_ending(word: &str) -> bool {
    if word.len() < 2 {
        return false;
    }

    let mut letters = word.chars().collect::<Vec<char>>();
    letters.pop().unwrap() == letters.pop().unwrap()
}

//ends in a constonant followed by a vowel followe dby a constonant
fn is_cvc_ending(word: &str) -> bool {
    if word.len() < 3 {
        return false
    }

    let reversed_letters: Vec<char>  =  word.chars().rev().collect();

    !is_vowel(reversed_letters[0], None) 
    && is_vowel(reversed_letters[1], None)
    && !is_vowel(reversed_letters[2], None) 
}