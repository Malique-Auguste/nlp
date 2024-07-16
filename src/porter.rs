pub fn stem(word: String) -> String {
    let m = calc_m(&word);
    
    unimplemented!()
}


fn calc_m(word: &str) -> u8 {
    if word.len() <= 2 {
        return 0
    }

    //list stating if a character in a word is a vowel or not
    let mut is_vowel_list = Vec::new();
    for letter in word.chars() {
        is_vowel_list.push(is_vowel(letter, is_vowel_list.last()))
    }

    let is_last_letter_vowel = is_vowel_list.pop().unwrap();

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

    m
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