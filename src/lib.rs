mod deck;

use deck::new_shuffled_deck;
pub use deck::Card;
pub use deck::CardSymbol;
pub use deck::CardValue;

pub struct GameTables {
    pub playing_table: [Vec<Card>; 7],
    pub foundation_table: [Vec<Card>; 4],
    pub extra_table: Vec<Card>,
    pub drawn_table: Vec<Card>,
}

pub enum GameAction {
    DrawCard,
    UnDraw,
    MoveToPlayingFromFoundation(usize, usize), //the foundation stack it move from to the playing stack
    MoveToFoundationFromPlaying(usize, usize), //the stack it moves from to the foundation stack it moves to

    MoveToPlayingFromDrawn(usize), //the stack to which it moves
    MoveToDrawnFromPlaying(usize),

    MoveToDrawFromFoundation(usize),
    MoveToFoundationFromDrawn(usize),

    MoveToPlayingFromPlaying(usize, usize, usize, usize), //the stack and row from which it moves to the stack and row it moves to

    ShowCard(usize), //show the top card in the stack (it assumed that it's hidden)
    HideCard(usize),
}

pub struct GameData {
    pub action_history: Vec<GameAction>,
    pub tables: GameTables,
}

fn can_follow_foundation_card(this: &Card, other: &Card) -> bool {
    this.symbol == other.symbol && other.value.is_followed_by(this.value)
}

fn can_follow_playing_card(this: &Card, other: &Card) -> bool {
    this.get_color() != other.get_color() && this.value.is_followed_by(other.value)
}

impl GameData {
    pub fn new(shuffle_times: i32) -> GameData {
        let mut deck = new_shuffled_deck(shuffle_times);

        let drawn_table: Vec<Card> = Vec::new();
        let foundation_table: [Vec<Card>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
        let mut playing_table = [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ];

        for (index, stack) in playing_table.iter_mut().enumerate() {
            for _ in 0..(index + 1) {
                stack.push(deck.pop().unwrap());
            }
            stack[index].is_face_up = true;
        }

        let extra_table = Vec::from(deck);
        let action_history: Vec<GameAction> = Vec::new();

        GameData {
            action_history,
            tables: GameTables {
                playing_table,
                foundation_table,
                extra_table,
                drawn_table,
            },
        }
    }

    pub fn do_(&mut self, action: GameAction) -> Result<GameAction, String> {
        match action {
            GameAction::DrawCard => self.draw(action),
            GameAction::UnDraw => self.undraw(action),

            GameAction::MoveToFoundationFromPlaying(p, f) => {
                self.move_to_foundation_from_playing(p, f)
            }
            GameAction::MoveToPlayingFromFoundation(p, f) => {
                self.move_to_playing_from_foundation(p, f)
            }

            GameAction::MoveToPlayingFromDrawn(p) => self.move_to_playing_from_drawn(p),
            GameAction::MoveToDrawnFromPlaying(p) => self.move_to_drawn_from_playing(p),

            GameAction::MoveToDrawFromFoundation(f) => self.move_to_draw_from_foundation(f),
            GameAction::MoveToFoundationFromDrawn(f) => self.move_to_foundation_from_drawn(f),

            GameAction::MoveToPlayingFromPlaying(fs, fc, ts, tc) => {
                self.move_to_playing_from_playing(fs, fc, ts, tc)
            }

            GameAction::ShowCard(p) => self.set_card_visibility(p, true),
            GameAction::HideCard(p) => self.set_card_visibility(p, false),
        }
    }

    pub fn undo(&mut self) {
        let action = match self.action_history.pop() {
            Some(a) => a,
            None => return,
        };

        let _ = match action {
            GameAction::DrawCard => self.undraw(action),
            GameAction::UnDraw => self.draw(action),

            GameAction::MoveToFoundationFromPlaying(p, f) => {
                self.move_to_playing_from_foundation(p, f)
            }
            GameAction::MoveToPlayingFromFoundation(p, f) => {
                self.move_to_foundation_from_playing(p, f)
            }

            GameAction::MoveToPlayingFromDrawn(p) => self.move_to_drawn_from_playing(p),
            GameAction::MoveToDrawnFromPlaying(p) => self.move_to_playing_from_drawn(p),

            GameAction::MoveToDrawFromFoundation(f) => self.move_to_foundation_from_drawn(f),
            GameAction::MoveToFoundationFromDrawn(f) => self.move_to_draw_from_foundation(f),

            GameAction::MoveToPlayingFromPlaying(fs, fc, ts, tc) => {
                self.force_to_playing_from_playing(ts, tc, fs, fc)
            }

            GameAction::ShowCard(p) => {
                self.set_card_visibility(p, false).expect("");
                self.undo();
                Err(String::new())
            }
            GameAction::HideCard(p) => {
                self.set_card_visibility(p, true).expect("");
                self.undo();
                Err(String::new())
            }
        };
    }

    fn move_to_foundation_from_drawn(
        &mut self,
        foundation_stack: usize,
    ) -> Result<GameAction, String> {
        if self.tables.drawn_table.len() == 0 {
            return Err(String::from("Draw Stack is empty!"));
        }

        let contender_card = self
            .tables
            .drawn_table
            .pop()
            .expect("We check for non-zero length before");

        if self.tables.foundation_table[foundation_stack].len() == 0 {
            if contender_card.value == CardValue::Ace {
                self.tables.foundation_table[foundation_stack].push(contender_card);
                return Ok(GameAction::MoveToFoundationFromDrawn(foundation_stack));
            } else {
                self.tables.drawn_table.push(contender_card);
                return Err(format!(
                    "Card {} cannot go on foundation stack {}",
                    contender_card, foundation_stack
                ));
            }
        }

        let receiver_card = self.tables.foundation_table[foundation_stack]
            .get(self.tables.foundation_table[foundation_stack].len() - 1)
            .expect("We check for non-zero length before");

        if !can_follow_foundation_card(&contender_card, receiver_card) {
            self.tables.drawn_table.push(contender_card);
            return Err(format!(
                "Card {} cannot follow card {}",
                contender_card, receiver_card
            ));
        }

        self.tables.foundation_table[foundation_stack].push(contender_card);
        Ok(GameAction::MoveToFoundationFromDrawn(foundation_stack))
    }

    fn move_to_draw_from_foundation(
        &mut self,
        foundation_stack: usize,
    ) -> Result<GameAction, String> {
        match self.tables.foundation_table[foundation_stack].pop() {
            Some(c) => {
                self.tables.drawn_table.push(c);
                Ok(GameAction::MoveToDrawFromFoundation(foundation_stack))
            }
            None => Err(String::from("Illegal Move!")),
        }
    }
    fn draw(&mut self, action: GameAction) -> Result<GameAction, String> {
        if self.tables.extra_table.is_empty() && self.tables.drawn_table.is_empty() {
            return Ok(action);
        }

        if self.tables.extra_table.is_empty() {
            std::mem::swap(&mut self.tables.extra_table, &mut self.tables.drawn_table);
            self.tables.extra_table.reverse();
            for card in self.tables.extra_table.iter_mut() {
                card.is_face_up = false;
            }
            return Ok(action);
        } else {
            let mut card = self.tables.extra_table.pop().unwrap();
            card.is_face_up = true;
            self.tables.drawn_table.push(card);
            return Ok(action);
        }
    }

    fn undraw(&mut self, action: GameAction) -> Result<GameAction, String> {
        if self.tables.extra_table.is_empty() && self.tables.drawn_table.is_empty() {
            return Ok(action);
        }

        if self.tables.drawn_table.is_empty() {
            std::mem::swap(&mut self.tables.extra_table, &mut self.tables.drawn_table);
            self.tables.drawn_table.reverse();
            for card in self.tables.drawn_table.iter_mut() {
                card.is_face_up = true;
            }
            return Ok(action);
        } else {
            let mut card = self.tables.drawn_table.pop().unwrap();
            card.is_face_up = false;
            self.tables.extra_table.push(card);
            return Ok(action);
        }
    }

    fn move_to_foundation_from_playing(
        &mut self,
        playing_stack: usize,
        foundation_stack: usize,
    ) -> Result<GameAction, String> {
        let contender_card = match self.tables.playing_table[playing_stack].pop() {
            Some(x) => x,
            None => return Err(String::from("Card stack is empty!")),
        };

        if self.tables.foundation_table[foundation_stack].len() == 0 {
            if contender_card.value == CardValue::Ace {
                self.tables.foundation_table[foundation_stack].push(contender_card);
                return Ok(GameAction::MoveToFoundationFromPlaying(
                    playing_stack,
                    foundation_stack,
                ));
            } else {
                self.tables.playing_table[playing_stack].push(contender_card);
                return Err(String::from("Foundation Stack is empty!"));
            }
        }

        if let Some(card) = self.tables.foundation_table[foundation_stack]
            .get(self.tables.foundation_table[foundation_stack].len() - 1)
        {
            if !can_follow_foundation_card(&contender_card, card) {
                self.tables.playing_table[playing_stack].push(contender_card);
                return Err(String::from("Invalid move!"));
            }
        };

        // success! time to add the card
        self.tables.foundation_table[foundation_stack].push(contender_card);
        return Ok(GameAction::MoveToFoundationFromPlaying(
            playing_stack,
            foundation_stack,
        ));
    }

    fn move_to_playing_from_foundation(
        &mut self,
        playing_stack: usize,
        foundation_stack: usize,
    ) -> Result<GameAction, String> {
        let contender_card = match self.tables.foundation_table[foundation_stack].pop() {
            Some(x) => x,
            None => return Err(String::from("Card stack is empty!")),
        };

        if let Some(card) = self.tables.playing_table[playing_stack]
            .get(self.tables.playing_table[playing_stack].len() - 1)
        {
            if !can_follow_playing_card(&contender_card, card) {
                self.tables.foundation_table[foundation_stack].push(contender_card);
                return Err(String::from("Invalid Move!"));
            }
        };

        // success! time to add the card
        self.tables.playing_table[playing_stack].push(contender_card);
        return Ok(GameAction::MoveToPlayingFromFoundation(
            playing_stack,
            foundation_stack,
        ));
    }

    fn move_to_playing_from_drawn(&mut self, playing_stack: usize) -> Result<GameAction, String> {
        let contender_card = match self.tables.drawn_table.pop() {
            Some(card) => card,
            None => return Err(String::from("Card stack is empty!")),
        };

        if self.tables.playing_table[playing_stack].is_empty() {
            if contender_card.value == CardValue::King {
                self.tables.playing_table[playing_stack].push(contender_card);
                return Ok(GameAction::MoveToPlayingFromDrawn(playing_stack));
            } else {
                self.tables.drawn_table.push(contender_card);
                return Err(String::from("Illegal move"));
            }
        }

        let card = self.tables.playing_table[playing_stack]
            .get(self.tables.playing_table[playing_stack].len() - 1)
            .unwrap();

        if !can_follow_playing_card(&contender_card, card) {
            self.tables.drawn_table.push(contender_card);
            return Err(String::from("Illegal move"));
        }

        // now we are in the happy case
        self.tables.playing_table[playing_stack].push(contender_card);
        return Ok(GameAction::MoveToPlayingFromDrawn(playing_stack));
    }

    fn move_to_drawn_from_playing(&mut self, playing_stack: usize) -> Result<GameAction, String> {
        let contender_card = match self.tables.playing_table[playing_stack].pop() {
            Some(card) => card,
            None => return Err(String::from("Card stack is empty!")),
        };

        self.tables.drawn_table.push(contender_card);
        return Ok(GameAction::MoveToDrawnFromPlaying(playing_stack));
    }

    fn force_to_playing_from_playing(
        &mut self,
        from_stack: usize,
        from_card: usize,
        to_stack: usize,
        to_card: usize,
    ) -> Result<GameAction, String> {
        while self.tables.playing_table[from_stack].len() > from_card {
            let to_transfer = self.tables.playing_table[from_stack].remove(from_card);
            self.tables.playing_table[to_stack].push(to_transfer);
        }
        Ok(GameAction::MoveToPlayingFromPlaying(
            from_stack, from_card, to_stack, to_card,
        ))
    }
    fn move_to_playing_from_playing(
        &mut self,
        from_stack: usize,
        from_card: usize,
        to_stack: usize,
        to_card: usize,
    ) -> Result<GameAction, String> {
        if from_stack == to_stack {
            return Err(String::from("Cannot move to the same stack!"));
        }
        let _to_len = self.tables.playing_table[to_stack].len();

        if to_card != self.tables.playing_table[to_stack].len() {
            return Err(String::from(
                "Illegal card indexes, to_card index should be {to_len}",
            ));
        }

        let contender_card = match self.tables.playing_table[from_stack].get(from_card) {
            Some(card) => card,
            None => {
                return Err(String::from(
                    "Illegal card indexes, from_card index out of bounds",
                ))
            }
        };

        if !contender_card.is_face_up {
            return Err(String::from("Card shold be face up!!"));
        }

        if self.tables.playing_table[to_stack].len() == 0 {
            if contender_card.value == CardValue::King {
                while self.tables.playing_table[from_stack].len() > from_card {
                    let to_transfer = self.tables.playing_table[from_stack].remove(from_card);
                    self.tables.playing_table[to_stack].push(to_transfer);
                }
                return Ok(GameAction::MoveToPlayingFromPlaying(
                    from_stack, from_card, to_stack, to_card,
                ));
            } else {
                return Err(String::from("Illegall Move!"));
            }
        }

        let receiver_card = match self.tables.playing_table[to_stack].get(to_card - 1) {
            Some(card) => card,
            None => {
                return Err(format!(
                    "Illegal move, card with index {} on stack {} does not exist!",
                    to_card - 1,
                    to_stack
                ));
            }
        };

        if !can_follow_playing_card(contender_card, receiver_card) {
            return Err(format!(
                "Illegal Move! {:?} cannot be on top of {:?}",
                contender_card, receiver_card
            ));
        }

        while self.tables.playing_table[from_stack].len() > from_card {
            let to_transfer = self.tables.playing_table[from_stack].remove(from_card);
            self.tables.playing_table[to_stack].push(to_transfer);
        }
        Ok(GameAction::MoveToPlayingFromPlaying(
            from_stack, from_card, to_stack, to_card,
        ))
    }

    fn set_card_visibility(
        &mut self,
        playing_stack: usize,
        visible: bool,
    ) -> Result<GameAction, String> {
        let lenght = self.tables.playing_table[playing_stack].len();

        if self.tables.playing_table[playing_stack].len() == 0 {
            return Err(String::from("Stack is empty!!!"));
        }
        let mut card = match self.tables.playing_table[playing_stack].get_mut(lenght - 1) {
            Some(c) => c,
            None => return Err(String::from("Stack of cards empty!")),
        };

        if card.is_face_up == visible {
            return Err(String::from("Card already visible {visibility}"));
        }

        card.is_face_up = visible;

        if visible {
            Ok(GameAction::ShowCard(playing_stack))
        } else {
            Ok(GameAction::HideCard(playing_stack))
        }
    }
}
