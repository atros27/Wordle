use std::collections::HashMap;
use std::sync::LazyLock;
use iced::Color;

pub(crate) struct WordSet {
    pub words: Vec<String>,
    pub answer_words: Vec<String>,
}

impl WordSet {
    pub fn suggest(&self) -> (String, HashMap<String,f64>) {
        if self.answer_words.len() == 1 {
            let answer = self.answer_words[0].clone().to_ascii_uppercase(); 
            let mut map = HashMap::new();
            map.insert(answer.clone(), 1.0);
            return (answer, map)}
        let mut heuristic_table = HashMap::new();
        let mut ans = "".to_string();
        let mut peak_heuristic = 0.0;
        for guess_word in &self.words {
            let mut probability = [0; 3i32.pow(6) as usize];
            for answer_word in &self.answer_words {
                let mut bin_num = 0;
                for i in 0..5 {
                    if guess_word.chars().nth(i) == answer_word.chars().nth(i) {
                        bin_num += 2 * (3u32).pow(i as u32);
                    } else if answer_word.contains(guess_word.chars().nth(i).unwrap()) {
                        bin_num += 1 * (3u32).pow(i as u32);
                    }
                }
                probability[bin_num as usize] += 1;
            }
            let mut exp_info = 0.0;
            for i in 0..(3i32.pow(6) as usize) {
                let p = (probability[i] as f64) / (self.answer_words.len() as f64);
                if probability[i] > 0 {exp_info += p * (1.0 / p).log(2.0)}
            }
            heuristic_table.insert(guess_word.clone().to_ascii_uppercase(), exp_info);
            if exp_info > peak_heuristic {
                println!("{:.2}",&exp_info);
                peak_heuristic = exp_info;
                ans = guess_word.clone();
            }
        }
        println!("Expected Info: {:.2}",peak_heuristic);
        (ans.to_ascii_uppercase(), heuristic_table)
    }
    pub fn reduce(&mut self, grade_result: [(char, Color); 5]) {
        let YELLOW = Color::from_rgb8(0xFF, 0xCE, 0x1B);
        let GREEN = Color::from_rgb8(0x04, 0x63, 0x07);
        let mut answer_set = Vec::new();
        for word in &self.answer_words {
            // println!("WORD: {}",&word);
            let mut passes_check = true;
            //if *word == *guess {passes_check = false;}
            if word.chars().zip(grade_result).all(|(word_char, (guess_char, color))| word_char == guess_char) {passes_check = false;}
            else {
                for i in 0..5 {
                    match grade_result[i].1 {
                        color if color == GREEN => {
                            if word.chars().nth(i) != Some(grade_result[i].0.to_ascii_lowercase()) {passes_check = false;break}
                        },
                        color if color == YELLOW => {
                            if !word.contains(grade_result[i].0.to_ascii_lowercase()) ||
                                word.chars().nth(i) == Some(grade_result[i].0.to_ascii_lowercase()) {passes_check = false;break}
                        },
                        _ => {
                            if word.contains(grade_result[i].0.to_ascii_lowercase()) {passes_check = false;break}
                        },
                    }
                }
            }
            if passes_check {answer_set.push(word.clone());}
            //  if word.chars().zip(grade_result).all(|(word_char, (guess_char, color))|
            // match color {
            //     color if color == GREEN => word_char == guess_char,
            //     color if color == YELLOW => word.contains(guess_char),
            //     _ => !word.contains(guess_char)
            // }) {answer_set.push(word.clone());}
        }
        self.answer_words = answer_set;
        //WordSet {words: self.words.clone(), answer_words: answer_set}
    }
}