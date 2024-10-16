// Blackjack, by Al Sweigart al@inventwithpython.com
// The classic card game also known as 21. (This version doesn't have
// splitting or insurance.)
// More info at: https://en.wikipedia.org/wiki/Blackjack
// This code is available at https://nostarch.com/big-book-small-python-programming
// Tags: large, game, card game

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::char;
use std::cmp;
use std::cmp::Ordering;
use std::fmt;
use std::io::stdin;
use std::u8;
use std::vec;

#[derive(Clone, Copy, PartialEq, Debug)]
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl Suit {
    const VALUES: [Self; 4] = [Self::Heart, Self::Diamond, Self::Spade, Self::Club];
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "{}", char::from_u32(9829).unwrap()),
            Suit::Diamond => write!(f, "{}", char::from_u32(9830).unwrap()),
            Suit::Spade => write!(f, "{}", char::from_u32(9824).unwrap()),
            Suit::Club => write!(f, "{}", char::from_u32(9827).unwrap()),
        }
    }
}
#[derive(Clone, Copy, PartialEq, Debug)]
enum Rank {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    const FACES: [Self; 4] = [Self::Jack, Self::Queen, Self::King, Self::Ace];
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Rank::Number(x) => match x.cmp(&10) {
                Ordering::Less => write!(f, "{} ", x),
                Ordering::Equal => write!(f, "{}", x),
                Ordering::Greater => Ok(()),
            },
            Rank::Jack => write!(f, "J "),
            Rank::Queen => write!(f, "Q "),
            Rank::King => write!(f, "K "),
            Rank::Ace => write!(f, "A "),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Visibility {
    Hidden,
    Visible,
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Hidden
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Card {
    suit: Suit,
    rank: Rank,
    state: Visibility,
}

fn main() {
    println!(
        "Blackjack, by Al Sweigart al@inventwithpython.com
Rules:
Try to get as close to 21 without going over.
Kings, Queens, and Jacks are worth 10 points.
Aces are worth 1 or 11 points.
Cards 2 through 10 are worth their face value.
(H)it to take another card.
(S)tand to stop taking cards.
On your first play, you can (D)ouble down to increase your bet
but must hit exactly one more time before standing.
In case of a tie, the bet is returned to the player.
The dealer stops hitting at 17."
    );

    let mut _money: i64 = 5000;
    let mut deck = get_deck();

    'game: loop {
        if _money <= 0 {
            println!("You're broke!\nGood thing you weren't playing with real money.\nThanks for playing!\n");
            break;
        }

        println!("Available funds: {}", _money);
        let mut bet = get_bet(_money);

        let mut dealer_hand = draw_cards(&mut deck, None, None, 1, false);
        dealer_hand.push(draw_cards(&mut deck, None, None, 1, true)[0]);

        let mut player_hand = draw_cards(&mut deck, None, None, 2, true);

        'player_action: loop {
            display_hands(&player_hand, &dealer_hand, false);

            if get_hand_value(&player_hand) > 21 {
                break 'player_action;
            }

            let selected_move = get_move(&player_hand, _money - bet);

            if selected_move == "D".to_string() {
                bet += get_bet(cmp::min(bet, _money - bet));
                println!("Bet increased to {}", bet);
                println!("Bet: {}", bet);
            }

            if selected_move == "H".to_string() || selected_move == "D" {
                let new_card =
                    draw_cards(&mut deck, Some(&player_hand), Some(&dealer_hand), 1, true)[0];
                player_hand.push(new_card);
                println!("You drew a {} of {}", new_card.rank, new_card.suit);
                println!("Bet: {bet}");
            }

            if selected_move == "S" || selected_move == "D" {
                break 'player_action;
            }
        }

        if get_hand_value(&dealer_hand) <= 21 {
            while get_hand_value(&dealer_hand) < 17 {
                println!("Dealer hits...");
                let new_card =
                    draw_cards(&mut deck, Some(&player_hand), Some(&dealer_hand), 1, true)[0];
                dealer_hand.push(new_card);

                display_hands(&player_hand, &dealer_hand, false);

                if get_hand_value(&dealer_hand) > 21 {
                    break;
                }
            }
        }

        println!("Any input to reveal hands...");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.contains('\n') {
            println!("\n\n");
        }

        dealer_hand[0].state = Visibility::Visible;
        display_hands(&player_hand, &dealer_hand, true);
        let player_value = get_hand_value(&player_hand);
        let dealer_value = get_hand_value(&dealer_hand);

        if player_value > 21 {
            println!("You bust!");
            _money -= bet;
        } else if dealer_value > 21 {
            println!("Dealer busts! You win ${}", bet);
            _money += bet;
        } else {
            match dealer_value.cmp(&player_value) {
                Ordering::Equal => {
                    println!("It's a tie, the bet is returned to you.")
                }
                Ordering::Greater => {
                    println!("You lost!");
                    _money -= bet;
                }
                Ordering::Less => {
                    println!("You won ${}", bet);
                    _money += bet;
                }
            }
        }
        loop {
            println!("[C]ontinue or [Q]uit?");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            match input.trim().to_uppercase().as_str() {
                "C" => {
                    break;
                }
                "Q" => break 'game,
                _ => {
                    println!("Enter a valid input.");
                    ()
                }
            }
        }
    }
}

fn get_bet(max_bet: i64) -> i64 {
    let mut bet_invalid: bool = true;
    let mut bet: i64 = 0;

    while bet_invalid {
        println!("How much do you bet? (1-{})", max_bet);

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        bet = input.trim().parse::<i64>().unwrap_or(0);

        if bet > 0 && bet <= max_bet {
            bet_invalid = false;
        } else {
            println!("Invalid bet - retry.")
        }
    }
    bet
}

fn get_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = vec![];

    for rank in 2..11 {
        Suit::VALUES.map(|suit| {
            deck.push(Card {
                suit: suit.clone(),
                rank: Rank::Number(rank),
                state: Visibility::Hidden,
            });
        });
    }
    for suit in Suit::VALUES {
        Rank::FACES.map(|face| {
            deck.push(Card {
                suit: suit.clone(),
                rank: face,
                state: Visibility::Hidden,
            })
        });
    }

    let mut rng = thread_rng();
    deck.shuffle(&mut rng);
    deck
}

fn display_hands(player_hand: &Vec<Card>, dealer_hand: &Vec<Card>, show_dealer_hand: bool) {
    if show_dealer_hand {
        println!("DEALER: {}", get_hand_value(dealer_hand));
    } else {
        println!("DEALER: ???");
    }
    display_cards(dealer_hand);

    println!("PLAYER: {}", get_hand_value(player_hand));

    display_cards(player_hand);
}

fn display_cards(cards: &Vec<Card>) {
    let _ = cards
        .iter()
        .map(move |c| match c.state {
            Visibility::Hidden => {
                println!(" ___  \n|## | \n|###| \n|_##| ");
            }
            Visibility::Visible => {
                println!(" ___  \n|{} | \n| {} | \n|{} | ", c.rank, c.suit, c.rank,);
            }
        })
        .collect::<Vec<_>>();
}

fn get_hand_value(cards: &Vec<Card>) -> u8 {
    let mut value: u8 = 0;
    let mut ace_count: u8 = 0;

    for card in cards {
        match card.rank {
            Rank::Number(x) => value += x,
            Rank::Jack => value += 10,
            Rank::Queen => value += 10,
            Rank::King => value += 10,
            Rank::Ace => ace_count += 1,
        }
    }
    value += ace_count;

    for _ in 1..=ace_count {
        if value + 10 <= 21 {
            value += 10
        }
    }
    value
}

fn get_move(player_hand: &Vec<Card>, _money: i64) -> String {
    loop {
        let mut moves = vec!["(H)it", "(S)tand"];

        if player_hand.len() == 2 && _money > 0 {
            moves.push("(D)ouble down");
        }
        println!("{}{}", moves.join(", "), ">");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim().to_uppercase().as_str() {
            "H" => return "H".to_string(),
            "S" => return "S".to_string(),
            "D" => return "D".to_string(),
            _ => {
                println!("Enter a valid input.");
                ()
            }
        }
    }
}

fn draw_cards(
    deck: &mut Vec<Card>,
    player_hand: Option<&Vec<Card>>,
    dealer_hand: Option<&Vec<Card>>,
    draw_count: u8,
    visible: bool,
) -> Vec<Card> {
    let mut drawn_cards: Vec<Card> = vec![];

    for _ in 0..draw_count {
        if deck.is_empty() {
            *deck =
                shuffle_discarded_into_new_deck(deck, player_hand.unwrap(), dealer_hand.unwrap())
        }

        let mut card = deck.pop().unwrap();

        if visible {
            card.state = Visibility::Visible;
        }

        drawn_cards.push(card);
    }
    drawn_cards
}

fn shuffle_discarded_into_new_deck(
    deck: &Vec<Card>,
    player_hand: &Vec<Card>,
    dealer_hand: &Vec<Card>,
) -> Vec<Card> {
    let cards = get_deck()
        .iter()
        .filter(|x| {
            !player_hand.contains(x) || !dealer_hand.contains(x) || !deck.clone().contains(x)
        })
        .cloned()
        .collect();
    cards
}
