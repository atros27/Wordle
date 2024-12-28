use std::collections::BTreeMap;
use std::sync::LazyLock;
use iced::{Element, Color, Padding};
use iced::widget::{Container, container, row, column, text, Row};

pub static GREY: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x88, 0x88, 0x88));
pub static DARK_GREY: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x44, 0x44, 0x44));
pub static YELLOW: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0xFF, 0xCE, 0x1B));
pub static GREEN: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x04, 0x63, 0x07));
pub static BLACK: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0x00, 0x00, 0x00));
pub static WHITE: LazyLock<Color, fn() -> Color> = LazyLock::new(|| Color::from_rgb8(0xFF, 0xFF, 0xFF));

#[derive(Debug)]
enum Message {
    EnterText(char),
    DeleteText,
    Enter,
}

#[derive(Copy, Clone)]
struct Entry {
    chars: [char; 5],
    colors: [Color; 5],
    is_active: bool,
    cursor: u8,
}

impl Default for Entry {
    fn default() -> Entry {
        Entry {
            chars: ['A'; 5],
            colors: [*GREY; 5],
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
    fn update (&mut self, message: Message) {}
}

// #[derive(Default)]
struct EntrySet {
    entries: [Entry; 6],
    strings: [[char; 5]; 6],
}

impl Default for EntrySet {
    fn default() -> EntrySet {
        EntrySet {
            entries: [Entry::default(); 6],
            strings: [[' '; 5]; 6]
        }
    }
}

impl EntrySet {
    fn from_strings(strings: [[char; 5]; 6]) -> EntrySet {
        let mut entries = [Entry::default(); 6];
        for i in 0..strings.len() {
            entries[i] = Entry::from_chars(strings[i]);
        }
        EntrySet {entries, strings}
    }
    fn view(&self) -> Element<'_, Message> {
        column![self.entries[0].view(),
        self.entries[1].view(),
        self.entries[2].view(),
        self.entries[3].view(),
        self.entries[4].view(),
        self.entries[5].view()]
            .spacing(10)
            .padding(Padding {top:100.0, right: 0.0, bottom: 0.0, left: 350.0})
            .into()
    }
    fn update(&mut self, message: Message) {}
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
            println!("{}",&c);
            row1_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        for i in 0..row2_chars.len() {
            let c = row2_chars.chars().nth(i).unwrap();
            println!("{}",&c);
            row2_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        for i in 0..row3_chars.len() {
            let c = row3_chars.chars().nth(i).unwrap();
            println!("{}",&c);
            row3_set.push(letter_block(c, self.state.get(&c).unwrap().clone()).into())
        }
        let row1 = Row::from_vec(row1_set);
        let row2 = Row::with_children(row2_set);
        let row3 = Row::with_children(row3_set);
        column![row1, row2, row3].into()
    }
}

#[derive(Default)]
struct Layout {
    entry_set: EntrySet,
    keyboard: Keyboard,
}

impl Layout {
    fn view(&self) -> Element<'_, Message> {
        column![self.entry_set.view(),self.keyboard.view()].into()
    }
    fn update(&mut self, message: Message) {}
}

fn letter_block(c: char, color: Color) -> Container<'static, Message> {
    container(text(c.to_ascii_uppercase().to_string()).size(50))
        //.padding(1)
        .center(55)
        .style(move |theme| {let mut x = container::rounded_box(theme); x.background = Some(color.into()); x})
        .into()
}

fn main() -> iced::Result {
    //let entry = Entry::from_strings(['h','e','l','l','o']);
    iced::run("Wordle in Rust!", Layout::update, Layout::view)
}
