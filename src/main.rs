mod suggestion;
use crate::suggestion::WordSet;

use std::collections::BTreeMap;
use std::fs;
use std::sync::LazyLock;
use iced::{Element, Color, Padding, Subscription, keyboard};
use iced::font::{Font, Weight};
use iced::widget::{Container, container, row, column, text, Row};
use iced::keyboard::{Key, Modifiers, key::Named};
use rand::seq::IteratorRandom;

pub static GREY: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x88, 0x88, 0x88));
pub static DARK_GREY: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x44, 0x44, 0x44));
pub static RED: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0xFF, 0x45, 0x00));
pub static YELLOW: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0xFF, 0xCE, 0x1B));
pub static GREEN: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x04, 0x63, 0x07));
pub static BLACK: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x00, 0x00, 0x00));
pub static WHITE: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0xFF, 0xFF, 0xFF));

#[derive(Debug, Clone)]
enum GameResult {
    WIN,
    LOSE,
}

#[derive(Debug, Clone)]
enum Message {
    EnterText(char),
    DeleteText,
    Enter,
    GameOver(GameResult),
}

#[derive(Copy, Clone)]
struct Entry {
    chars: [char; 5],
    colors: [Color; 5],
    is_active: bool,
    cursor: usize,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            chars: [' '; 5],
            colors: [*DARK_GREY; 5],
            is_active: false,
            cursor: 0,
        }
    }
}

impl Entry {
    fn from_chars(chars: [char; 5]) -> Entry {
        Entry {
            chars,
            ..Entry::default()
        }
    }
    fn view(&self) -> Element<'_, Message> {
        row![letter_block(self.chars[0],self.colors[0]),
        letter_block(self.chars[1],self.colors[1]),
        letter_block(self.chars[2],self.colors[2]),
        letter_block(self.chars[3],self.colors[3]),
        letter_block(self.chars[4],self.colors[4])]
            .spacing(5)
            .into()
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::EnterText(c) => {
                if c.is_ascii_alphabetic() && self.cursor < 5 {
                    self.chars[self.cursor] = c;
                    self.cursor += 1;
                }
            },
            Message::DeleteText => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                    self.chars[self.cursor] = ' ';
                }
            },
            _ => {},
        }
    }

}

// #[derive(Default)]
struct EntrySet {
    entries: [Entry; 6],
    strings: [[char; 5]; 6],
    active_entry: usize,
    secret_word: String,
    suggestion_word_bank: WordSet,
}

impl Default for EntrySet {
    fn default() -> EntrySet {
        let mut ans = EntrySet {
            entries: [Entry::default(); 6],
            strings: [[' '; 5]; 6],
            active_entry: 0,
            secret_word: fs::read_to_string("sgb-words-trimmed.txt")
                .unwrap().split('\n')
                .choose(&mut rand::thread_rng())
                .unwrap().to_string().to_ascii_uppercase(),
            suggestion_word_bank: WordSet {
                words: fs::read_to_string("sgb-words.txt")
                .unwrap().split('\n').into_iter().map(|x| x.to_string()).collect()
            }
        };
        //println!("Suggestion: {}",ans.suggestion_word_bank.suggest());
        println!("Suggestion: tares");
        for i in 0..5 {
            ans.entries[0].colors[i] = *GREY;
        }
        ans
    }
}

impl EntrySet {
    // fn from_strings(strings: [[char; 5]; 6]) -> EntrySet {
    //     let mut entries = [Entry::default(); 6];
    //     for i in 0..strings.len() {
    //         entries[i] = Entry::from_chars(strings[i]);
    //     }
    //     EntrySet {entries, strings, active_entry: 0}
    // }
    fn view(&self) -> Element<'_, Message> {
        column![self.entries[0].view(),
        self.entries[1].view(),
        self.entries[2].view(),
        self.entries[3].view(),
        self.entries[4].view(),
        self.entries[5].view()]
            .spacing(10)
            .padding(Padding {top:20.0, right: 0.0, bottom: 0.0, left: 350.0})
            .into()
    }
    fn grade(&mut self, grade_result: [(char, Color); 5]) {
        self.entries[self.active_entry].colors = grade_result.map(|(_, color)| color);
        // println!(
        //     "Grade: {:?} {:?} {:?} {:?} {:?}",
        //     self.colors[0], self.colors[1], self.colors[2], self.colors[3], self.colors[4]`
        // );
        let old_bank_length = self.suggestion_word_bank.words.len();
        self.suggestion_word_bank = self.suggestion_word_bank.reduce(grade_result);
        println!("Reduced from {} words to {} words", old_bank_length, self.suggestion_word_bank.words.len());
        println!("Suggestion: {}",self.suggestion_word_bank.suggest());
    }
}

struct Keyboard {
    state: BTreeMap<char, Color>,
}

impl Default for Keyboard {
    fn default() -> Self {
        let mut state = BTreeMap::new();
        for c in 'A'..='Z' {
            state.insert(c, *GREY);
        }
        Self {state}
    }
}

impl Keyboard {
    fn view(&self) -> Element<'_, Message> {
        let row1_chars = "QWERTYUIOP".to_string();
        let row2_chars = "ASDFGHJKL".to_string();
        let row3_chars = "ZXCVBNM".to_string();

        let mut row1_set: Vec<Element<Message>> = Vec::new();
        let mut row2_set: Vec<Element<Message>> = Vec::new();
        let mut row3_set: Vec<Element<Message>> = Vec::new();

        for i in 0..row1_chars.len() {
            let c = row1_chars.chars().nth(i).unwrap();
            row1_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        for i in 0..row2_chars.len() {
            let c = row2_chars.chars().nth(i).unwrap();
            row2_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        for i in 0..row3_chars.len() {
            let c = row3_chars.chars().nth(i).unwrap();
            row3_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        let row1 = Row::from_vec(row1_set).spacing(5)
            .padding(Padding {top: 30.0, right: 0.0, bottom: 0.0, left: 200.0});
        let row2 = Row::with_children(row2_set).spacing(5)
        .padding(Padding {top: 10.0, right: 0.0, bottom: 0.0, left: 230.0});
        let row3 = Row::with_children(row3_set).spacing(5)
            .padding(Padding {top: 10.0, right: 0.0, bottom: 0.0, left: 290.0});
        column![row1, row2, row3].into()
    }
    fn update(&mut self, message: Message) {}
    fn grade(&mut self, grade_result: [(char, Color); 5]) {
        for (c, color) in grade_result {
            self.state.insert(c, color);
        }
    }
}

struct Title {
    text: String,
    color: Color,
}

impl Default for Title {
    fn default() -> Self {
        Self {text: "Wordle".to_string(), color: *WHITE}
    }
}

impl Title {
    fn view(&self) -> Element<'_, Message> {
        container(text(self.text.clone()).size(50).color(self.color)
            .font(Font {weight: Weight::Bold, ..Font::default()}))
            .center_x(1000).padding(Padding {top: 20.0, right: 0.0, bottom: 0.0, left: 0.0}).into()
    }
}

#[derive(Default)]
struct Layout {
    title: Title,
    entry_set: EntrySet,
    keyboard: Keyboard,
}

impl Layout {
    fn view(&self) -> Element<'_, Message> {
        column![self.title.view(),self.entry_set.view(),self.keyboard.view()].into()
    }
    fn grade(&mut self, word: [char; 5]) -> [(char, Color); 5] {
        let mut ans = [('A', *GREY); 5];
        for i in 0..5 {
            ans[i].0 = word[i];
            if word[i] == self.entry_set.secret_word.chars().nth(i).unwrap() {
                ans[i].1 = *GREEN;
            } else if self.entry_set.secret_word.contains(word[i]) {
                ans[i].1 = *YELLOW;
            } else {
                ans[i].1 = *DARK_GREY;
            }
        }
        ans
    }
    fn update(&mut self, message: Message) {
        match message {
            Message::EnterText(_c) => {self.entry_set.entries[self.entry_set.active_entry].update(message);},
            Message::DeleteText => {self.entry_set.entries[self.entry_set.active_entry].update(message);},
            Message::Enter => {
                if !self.entry_set.entries[self.entry_set.active_entry].chars.contains(&' ') {
                    let grade_result = self.grade(self.entry_set.entries[self.entry_set.active_entry].chars);
                    self.entry_set.grade(grade_result);
                    self.keyboard.grade(grade_result);
                    if grade_result.iter().all(|(_, color)| *color == *GREEN) {
                        self.update(Message::GameOver(GameResult::WIN));
                    } else if self.entry_set.active_entry == 5 {
                        self.update(Message::GameOver(GameResult::LOSE));
                    } else {
                        self.entry_set.active_entry += 1;
                        self.entry_set.entries[self.entry_set.active_entry].colors = [*GREY; 5];
                    }
                }
            }
            Message::GameOver(result) => {
                self.title.text = format!("Answer: {}", self.entry_set.secret_word);
                match result {
                    GameResult::WIN => self.title.color = *GREEN,
                    GameResult::LOSE => self.title.color = *RED,
                }
            },
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(parse_keyboard_input)
    }
}

fn letter_block(c: char, color: Color) -> Container<'static, Message> {
    container(text(c.to_ascii_uppercase().to_string()).size(50))
        .center(55)
        .style(move |theme| {let mut x = container::rounded_box(theme); x.background = Some(color.into()); x})
        .into()
}

fn parse_keyboard_input(key: Key, _modifiers: Modifiers) -> Option<Message> {
    match key {
        Key::Character(c) => Some(Message::EnterText(c.to_ascii_uppercase().parse().unwrap())),
        Key::Named(Named::Backspace) => Some(Message::DeleteText),
        Key::Named(Named::Enter) => Some(Message::Enter),
        _ => None
    }
}

fn main() -> iced::Result {
    //let entry = Entry::from_strings(['h','e','l','l','o']);
    iced::application("Wordle in Rust!", Layout::update, Layout::view)
        .subscription(Layout::subscription).run()
}
