use core::cmp::Ordering;
use rand::{thread_rng, rngs::ThreadRng, seq::SliceRandom};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
enum Suit {
    Clubs,    // zap, gato, zorro
    Hearts,   // copa
    Spades,   // espadilha
    Diamonds, // Ouros, Mole
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
enum Card {
    Three,
    Two,
    Ace,
    Knight,
    Joker,
    Queen,
    Seven,
    Six,
    Five,
    Four,
}

#[derive(Debug, PartialEq)]
struct CardWithSuit(Card, Suit);

impl CardWithSuit {
    fn is_manilha(&self, turned_card: &CardWithSuit) -> bool {
        (turned_card.0 as u8 + 1) == self.0 as u8
    }
}

impl PartialOrd for CardWithSuit {
    fn partial_cmp(&self, other: &CardWithSuit) -> Option<Ordering> {
        let Some(ord) = self.0.partial_cmp(&other.0) else { return None };

        match ord {
            Ordering::Equal => self.1.partial_cmp(&other.1),
            _ => Some(ord)
        }
    }
}



#[derive(Copy, Clone, PartialEq, PartialOrd)]
enum Turn {
    Player,
    Computer
}

struct Game {
    rng: ThreadRng,
    player_hand: Vec<CardWithSuit>,
    computer_hand: Vec<CardWithSuit>,
    turned_card: Option<CardWithSuit>,
    deck: Vec<CardWithSuit>,
    player_score: u8,
    computer_score: u8,
    turn: Turn,
    // Turn score is positive at the end, then player won, negative, computer won
    turn_score: i8,
    turn_stack: Vec<CardWithSuit>,
    score_increment: u8
}

impl Game {
    fn new() -> Game {
        Game {
            rng: thread_rng(),
            player_hand: vec![],
            computer_hand: vec![],
            turned_card: None,
            deck: vec![],
            player_score: 0,
            computer_score: 0,
            turn: Turn::Player,
            turn_score: 0,
            turn_stack: vec![],
            score_increment: 1,
        }
    }

    fn init(&mut self) {
        self.build_deck();
    }

    fn get_scores(&self) -> (u8, u8) {
        (self.player_score, self.computer_score)
    }

    fn build_deck(&mut self) {
        let suits: [Suit; 4] = [Suit::Diamonds, Suit::Spades, Suit::Clubs, Suit::Hearts];
        let cards: [Card; 10] = [
            Card::Three,
            Card::Two,
            Card::Ace,
            Card::Knight,
            Card::Joker,
            Card::Queen,
            Card::Seven,
            Card::Six,
            Card::Five,
            Card::Four
        ];

        for s in suits.iter() {
            for c in cards.iter() {
                self.deck.push(CardWithSuit(*c, *s));
            }
        }

        self.deck.shuffle(&mut self.rng);
    }

    fn build_hands_and_flip(&mut self) {
        for i in 0..6 {
            let card = self.deck.pop().unwrap();

            if is_odd(i) && self.turn == Turn::Player {
                self.computer_hand.push(card);
            } else {
                self.player_hand.push(card);
            }
        }

        println!("player hand: {:?}", self.player_hand);
        println!("computer hand: {:?}", self.computer_hand);

        self.turned_card = self.deck.pop();
    }

    fn take_computer_hand(&mut self) -> CardWithSuit {
        // for now just taking the last card in hand
        self.computer_hand.pop().unwrap()
    }

    fn check_who_won_hand(&mut self, drawn_card: &CardWithSuit) {
        let Some(last_drawn) = self.turn_stack.last() else { return };

        let turned = self.turned_card.as_ref().expect("Game not initialized");

        let drawn_or_pile: bool;

        if drawn_card.is_manilha(&turned) && last_drawn.is_manilha(&turned) {
            drawn_or_pile = last_drawn > drawn_card;
        } else {
            if drawn_card.is_manilha(&turned) {
                drawn_or_pile = true;
            } else if last_drawn.is_manilha(&turned) {
                drawn_or_pile = false;
            } else {
                drawn_or_pile = last_drawn > drawn_card;
            }
        }

        let player_won: bool; // true => Player, false => Computer

        if drawn_or_pile {
            match self.turn {
                Turn::Player => {
                    player_won = true;
                },
                Turn::Computer => {
                    player_won = false;
                }
            }
        } else {
            match self.turn {
                Turn::Player => {
                    player_won = false;
                },
                Turn::Computer => {
                    player_won = true;
                }
            }
        }

        if player_won {
            self.turn_score += 1;
        } else {
            self.turn_score -= 1;
        }
    }

    fn reset_turn(&mut self) {
        self.build_deck();
        self.turn_score = 0;
        self.computer_hand = vec![];
        self.player_hand = vec![];
        self.turned_card = None;
    }

    fn start() {
        println!("Iniciando...");
        println!("Jogando contra o computador...");

        let mut game = Game::new();

        game.init();

        // Main game loop
        loop {
            let (ps, cs) = game.get_scores();
            println!("Placar atual: Jogador: {} - Computador: {}", ps, cs);

            game.build_hands_and_flip();

            println!("Sua mão: {:?}", game.player_hand);

            println!("Carta virada: {:?}", game.turned_card);

            // Running the turn
            for hand_index in 0..6 {
                println!("A cartas jogadas foram: {:?}", game.turn_stack);

                let drawn_card = match game.turn {
                    Turn::Player => {
                        println!("Sua vez! Qual carta vai jogar? {:?}", game.player_hand);

                        let chosen_card_index =choose_card();
                        game.player_hand.swap_remove(chosen_card_index as usize - 1)                              
                    },
                    Turn::Computer => {
                        let computer_card = game.take_computer_hand();
                        println!("O computador jogou a carta {:?}", computer_card);
                        computer_card
                    }
                };

                if hand_index > 0 && is_odd(hand_index) {
                    game.check_who_won_hand(&drawn_card);
                    if game.turn_score >= 0 {
                        println!("Você ganhou essa mão!");
                    } else {
                        println!("O computador ganhou essa mão!");
                    }
                }

                game.turn_stack.push(drawn_card);

                game.turn = match game.turn {
                    Turn::Player => Turn::Computer,
                    Turn::Computer => Turn::Player,
                };
            }

            game.reset_turn();

            if game.turn_score >= 0 {
                game.player_score += 1;
            } else {
                game.computer_score += 1;
            }

            if game.player_score >= 12 {
                println!("Você ganhou! Parabéns!");
                break;
            } else if game.computer_score >= 12 {
                println!("O computador ganhou, mais sorte na próxima vez!");
                break;
            }
        }
    }
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn choose_card() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("input is not integer")
}

fn main() {
    println!("Bora jogar um truco?");

    Game::start()
}
