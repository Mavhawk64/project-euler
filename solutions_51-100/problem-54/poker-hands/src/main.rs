use itertools::Itertools;
use std::fmt;
use std::fs::read_to_string;

fn main() {
    let poker_hands = read_lines(
        "/home/maverick/repos/project-euler/solutions_51-100/problem-54/poker-hands/src/poker.txt",
    );

    let mut p1: u16 = 0;
    let mut p2: u16 = 0;

    for hand in &poker_hands {
        let cards: Vec<String> = hand.split(' ').map(String::from).collect();
        let mid = cards.len() / 2;

        let p1_hand = Hand::new(cards[..mid].iter().map(|s| Card::from_string(s)).collect());
        let p2_hand = Hand::new(cards[mid..].iter().map(|s| Card::from_string(s)).collect());

        println!("Player 1: {} -- {}", p1_hand.score_as_text(), p1_hand);
        println!("Player 2: {} -- {}", p2_hand.score_as_text(), p2_hand);

        match p1_hand.hand_value().cmp(&p2_hand.hand_value()) {
            std::cmp::Ordering::Greater => {
                println!("Player 1 wins!\n");
                p1 += 1;
            }
            std::cmp::Ordering::Less => {
                println!("Player 2 wins!\n");
                p2 += 1;
            }
            std::cmp::Ordering::Equal => println!("TIE!\n"),
        }
    }
    println!(
        "Player 1 won {} times, Player 2 won {} times. Total Games: {}",
        p1,
        p2,
        p1 + p2
    );
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

#[derive(Debug, Clone, Copy)]
struct Card {
    rank: usize,
    suit: usize,
}

impl Card {
    fn new(rank: usize, suit: usize) -> Self {
        Card { rank, suit }
    }

    fn from_string(s: &str) -> Self {
        let chars: Vec<char> = s.chars().collect();

        let rank = match chars[0] {
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("Invalid rank: {}", chars[0]),
        };

        let suit = match chars[1] {
            'C' => 0,
            'D' => 1,
            'S' => 2,
            'H' => 3,
            _ => panic!("Invalid suit: {}", chars[1]),
        };

        Card { rank, suit }
    }
}

const RANKS: [&str; 13] = [
    "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Jack", "Queen",
    "King", "Ace",
];
const SUITS: [&str; 4] = ["Clubs", "Diamonds", "Spades", "Hearts"];

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", RANKS[self.rank - 2], SUITS[self.suit])
    }
}

struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        Hand { cards }
    }

    fn score_as_text(&self) -> &str {
        if self.is_royal_flush() {
            "Royal Flush"
        } else if self.is_straight_flush() {
            "Straight Flush"
        } else if self.is_four_of_a_kind() {
            "Four of a Kind"
        } else if self.is_full_house() {
            "Full House"
        } else if self.is_flush() {
            "Flush"
        } else if self.is_straight() {
            "Straight"
        } else if self.is_three_of_a_kind() {
            "Three of a Kind"
        } else if self.is_two_pair() {
            "Two Pair"
        } else if self.is_one_pair() {
            "One Pair"
        } else {
            "High Card"
        }
    }

    fn hand_value(&self) -> (u32, Vec<usize>) {
        let hand_type = if self.is_royal_flush() {
            9
        } else if self.is_straight_flush() {
            8
        } else if self.is_four_of_a_kind() {
            7
        } else if self.is_full_house() {
            6
        } else if self.is_flush() {
            5
        } else if self.is_straight() {
            4
        } else if self.is_three_of_a_kind() {
            3
        } else if self.is_two_pair() {
            2
        } else if self.is_one_pair() {
            1
        } else {
            0
        };

        let tiebreakers = self.get_tiebreakers(hand_type);
        (hand_type, tiebreakers)
    }

    fn get_tiebreakers(&self, hand_type: u32) -> Vec<usize> {
        match hand_type {
            7 => {
                // Four of a kind
                vec![
                    self.is_n_of_a_kind(4).unwrap(),
                    self.is_n_of_a_kind(1).unwrap(),
                ]
            }
            6 => {
                // Full house
                vec![
                    self.is_n_of_a_kind(3).unwrap(),
                    self.is_n_of_a_kind(2).unwrap(),
                ]
            }
            3 => {
                // Three of a kind
                let trips_rank = self.is_n_of_a_kind(3).unwrap();
                let mut kickers: Vec<usize> = self
                    .cards
                    .iter()
                    .map(|c| c.rank)
                    .filter(|&r| r != trips_rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                vec![trips_rank, kickers[0], kickers[1]]
            }
            2 => {
                // Two pair
                let mut pairs: Vec<usize> = self
                    .cards
                    .iter()
                    .map(|c| c.rank)
                    .counts()
                    .into_iter()
                    .filter(|(_, count)| *count == 2)
                    .map(|(rank, _)| rank)
                    .collect();
                pairs.sort_by(|a, b| b.cmp(a));
                let kicker = self.is_n_of_a_kind(1).unwrap();
                vec![pairs[0], pairs[1], kicker]
            }
            1 => {
                // One pair
                let pair_rank = self.is_n_of_a_kind(2).unwrap();
                let mut kickers: Vec<usize> = self
                    .cards
                    .iter()
                    .map(|c| c.rank)
                    .filter(|&r| r != pair_rank)
                    .collect();
                kickers.sort_by(|a, b| b.cmp(a));
                vec![pair_rank, kickers[0], kickers[1], kickers[2]]
            }
            _ => {
                // High card, flush, straight, straight flush
                let mut ranks: Vec<usize> = self.cards.iter().map(|c| c.rank).collect();
                ranks.sort_by(|a, b| b.cmp(a));
                ranks
            }
        }
    }

    fn is_royal_flush(&self) -> bool {
        self.is_straight_flush() && self.get_min_rank() == 10
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.is_n_of_a_kind(4).is_some()
    }

    fn is_full_house(&self) -> bool {
        let counts: Vec<usize> = self.get_rank_counts();
        counts.contains(&3) && counts.contains(&2)
    }

    fn is_flush(&self) -> bool {
        self.cards.iter().map(|c| c.suit).all_equal()
    }

    fn is_straight(&self) -> bool {
        let mut ranks: Vec<usize> = self.cards.iter().map(|c| c.rank).collect();
        ranks.sort();

        // Check normal consecutive straight
        ranks.windows(2).all(|w| w[1] == w[0] + 1)
            // Check ace-low straight (A-2-3-4-5)
            || ranks == vec![2, 3, 4, 5, 14]
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.is_n_of_a_kind(3).is_some()
    }

    fn is_two_pair(&self) -> bool {
        self.get_rank_counts().iter().filter(|&&c| c == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.is_n_of_a_kind(2).is_some()
    }

    fn is_n_of_a_kind(&self, n: usize) -> Option<usize> {
        self.cards
            .iter()
            .map(|c| c.rank)
            .counts()
            .into_iter()
            .find(|(_, count)| *count == n)
            .map(|(rank, _)| rank)
    }

    fn get_rank_counts(&self) -> Vec<usize> {
        self.cards
            .iter()
            .map(|c| c.rank)
            .counts()
            .values()
            .copied()
            .collect()
    }

    fn get_min_rank(&self) -> usize {
        self.cards.iter().map(|c| c.rank).min().unwrap()
    }

    fn get_max_rank(&self) -> usize {
        self.cards.iter().map(|c| c.rank).max().unwrap()
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, card) in self.cards.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", card)?;
        }
        write!(f, "]")
    }
}
