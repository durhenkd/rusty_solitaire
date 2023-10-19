use std::fmt::Display;

use crossterm::style::Stylize;
use rand::{thread_rng, Rng};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CardSymbol {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for CardSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardSymbol::Clubs => write!(f, "{}", '\u{2663}'),
            CardSymbol::Diamonds => write!(f, "{}", '\u{2666}'),
            CardSymbol::Hearts => write!(f, "{}", '\u{2665}'),
            CardSymbol::Spades => write!(f, "{}", '\u{2660}'),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum CardValue {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Display for CardValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CardValue::Ace => write!(f, "{}", " A"),
            CardValue::Two => write!(f, "{}", " 2"),
            CardValue::Three => write!(f, "{}", " 3"),
            CardValue::Four => write!(f, "{}", " 4"),
            CardValue::Five => write!(f, "{}", " 5"),
            CardValue::Six => write!(f, "{}", " 6"),
            CardValue::Seven => write!(f, "{}", " 7"),
            CardValue::Eight => write!(f, "{}", " 8"),
            CardValue::Nine => write!(f, "{}", " 9"),
            CardValue::Ten => write!(f, "{}", "10"),
            CardValue::Jack => write!(f, "{}", " J"),
            CardValue::Queen => write!(f, "{}", " Q"),
            CardValue::King => write!(f, "{}", " K"),
        }
    }
}

const CARD_SYMBOLS: [CardSymbol; 4] = [
    CardSymbol::Clubs,
    CardSymbol::Diamonds,
    CardSymbol::Hearts,
    CardSymbol::Spades,
];
const CARD_VALUES: [CardValue; 13] = [
    CardValue::Ace,
    CardValue::Two,
    CardValue::Three,
    CardValue::Four,
    CardValue::Five,
    CardValue::Six,
    CardValue::Seven,
    CardValue::Eight,
    CardValue::Nine,
    CardValue::Ten,
    CardValue::Jack,
    CardValue::Queen,
    CardValue::King,
];

impl CardValue {
    pub fn is_followed_by(&self, other: CardValue) -> bool {
        match (self, other) {
            (CardValue::Ace, CardValue::Two) => true,
            (CardValue::Two, CardValue::Three) => true,
            (CardValue::Three, CardValue::Four) => true,
            (CardValue::Four, CardValue::Five) => true,
            (CardValue::Five, CardValue::Six) => true,
            (CardValue::Six, CardValue::Seven) => true,
            (CardValue::Seven, CardValue::Eight) => true,
            (CardValue::Eight, CardValue::Nine) => true,
            (CardValue::Nine, CardValue::Ten) => true,
            (CardValue::Ten, CardValue::Jack) => true,
            (CardValue::Jack, CardValue::Queen) => true,
            (CardValue::Queen, CardValue::King) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    pub value: CardValue,
    pub symbol: CardSymbol,
    pub is_face_up: bool,
}
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }

    fn ne(&self, other: &Self) -> bool {
        self.value != other.value
    }
}
impl PartialOrd for Card {
    fn ge(&self, other: &Self) -> bool {
        self.value >= other.value
    }

    fn gt(&self, other: &Self) -> bool {
        self.value > other.value
    }

    fn le(&self, other: &Self) -> bool {
        self.value <= other.value
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.lt(other) {
            return Some(std::cmp::Ordering::Less);
        } else if self.gt(other) {
            return Some(std::cmp::Ordering::Greater);
        } else {
            return Some(std::cmp::Ordering::Equal);
        }
    }
}
impl Eq for Card {}
impl Card {
    pub fn get_color(&self) -> bool {
        self.symbol == CardSymbol::Clubs || self.symbol == CardSymbol::Spades
    }
}
impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.is_face_up {
            return write!(f, "\u{2552}\u{2550}\u{2555}");
        }

        let value = if self.get_color() {
            self.value.to_string()
        } else {
            self.value.to_string().dark_red().to_string()
        };
        let symbol = if self.get_color() {
            self.symbol.to_string()
        } else {
            self.symbol.to_string().dark_red().to_string()
        };
        write!(f, "{}{}", value, symbol)
    }
}

fn new_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for value in CARD_VALUES {
        for color in CARD_SYMBOLS {
            deck.push(Card {
                value,
                symbol: color,
                is_face_up: false,
            });
        }
    }

    deck
}

pub fn new_shuffled_deck(shuffle_times: i32) -> Vec<Card> {
    let mut deck = new_deck();

    let mut generator = thread_rng();

    for _ in 0..shuffle_times {
        let pos_one = generator.gen_range(0..deck.len());
        let pos_two = generator.gen_range(0..deck.len());

        let temp = deck[pos_one];
        deck[pos_one] = deck[pos_two];
        deck[pos_two] = temp;
    }

    deck
}
