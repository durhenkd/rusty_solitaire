use solitaire::{Card, CardSymbol, CardValue, GameData, GameTables};

pub fn get_game_data() -> GameData {
    GameData {
        tables: GameTables {
            playing_table: [
                Vec::from([
                    Card {
                        value: CardValue::Two,
                        symbol: CardSymbol::Clubs,
                        is_face_up: true,
                    },
                    Card {
                        value: CardValue::Ace,
                        symbol: CardSymbol::Diamonds,
                        is_face_up: true,
                    },
                ]),
                Vec::from([Card {
                    value: CardValue::Two,
                    symbol: CardSymbol::Spades,
                    is_face_up: true,
                }]),
                Vec::from([Card {
                    value: CardValue::King,
                    symbol: CardSymbol::Clubs,
                    is_face_up: true,
                }]),
                Vec::new(),
                Vec::from([
                    Card {
                        value: CardValue::Queen,
                        symbol: CardSymbol::Hearts,
                        is_face_up: true,
                    },
                    Card {
                        value: CardValue::Jack,
                        symbol: CardSymbol::Spades,
                        is_face_up: true,
                    },
                    Card {
                        value: CardValue::Ten,
                        symbol: CardSymbol::Diamonds,
                        is_face_up: true,
                    },
                ]),
                Vec::from([Card {
                    value: CardValue::Eight,
                    symbol: CardSymbol::Spades,
                    is_face_up: true,
                }]),
                Vec::from([
                    Card {
                        value: CardValue::Two,
                        symbol: CardSymbol::Diamonds,
                        is_face_up: true,
                    },
                    Card {
                        value: CardValue::Ace,
                        symbol: CardSymbol::Spades,
                        is_face_up: true,
                    },
                ]),
            ],
            foundation_table: [
                Vec::from([Card {
                    value: CardValue::Nine,
                    symbol: CardSymbol::Diamonds,
                    is_face_up: true,
                }]),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            ],
            drawn_table: Vec::new(),
            extra_table: Vec::from([
                Card {
                    value: CardValue::Six,
                    symbol: CardSymbol::Clubs,
                    is_face_up: false,
                },
                Card {
                    value: CardValue::Seven,
                    symbol: CardSymbol::Hearts,
                    is_face_up: false,
                },
                Card {
                    value: CardValue::Five,
                    symbol: CardSymbol::Spades,
                    is_face_up: false,
                },
            ]),
        },
        action_history: Vec::new(),
    }
}
