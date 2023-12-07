use std::{fmt, fs, collections::HashSet, cmp::Ordering};

fn main() {

    let input = read_file();
    let test_input = "32T3K 765
                            T55J5 684
                            KK677 28
                            KTJJT 220
                            QQQJA 483";

    
    let mut hands:Vec<Hand> = input.lines().map(|line| {
        let parts = line.trim().split(" ").collect::<Vec<&str>>();
        Hand::new(parts[0].to_string(), parts[1].parse::<u32>().unwrap())
    }).collect(); 

    hands.sort();

    let winnings: u32 = hands.iter().enumerate().fold(0, |acc, (i, hand)| acc + hand.bid * (i as u32 + 1));

    println!("Winnings: {}", winnings);
}


#[derive(Eq, PartialEq, Debug)]
struct Card {
    symbol: char,
    value: u32,
}

impl Card {
    fn new(card: char) -> Card {
        let value = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            _ => card.to_digit(10).unwrap(),
        };

        Card { symbol:card, value }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
    type_: u8,
}

impl Hand {
    fn new(cards: String, bid: u32) -> Hand {
        let type_ = Hand::get_best_type(&cards);
        let card_vec:Vec<Card> = cards.chars().map(|c| Card::new(c)).collect();

        Hand { cards: card_vec, bid, type_ }
    }

    fn get_type(card_str:&String) -> u8 {

        let char_vec:Vec<char> = card_str.chars().collect();
        let cards:HashSet<char> = card_str.chars().collect();

        match cards.len() {
            0 | 1 => return 7,
            2 => {
                for c in &char_vec {
                    let amount = char_vec.iter().filter(|&x| *x == *c).count();
    
                    if amount == 4 || card_str.contains("J") {
                        return 6; // Three of a kind
                    }
                }
    
                // Full house
                return 5;
            },
            3 => {
                for c in &char_vec {
                    let amount = char_vec.iter().filter(|&x| *x == *c).count();
    
                    if amount == 3 || card_str.contains("J") {
                        return 4; // Three of a kind
                    }
                }
    
                return 3; // Two pair
            },
            4 => return 2,
            _ => return 1,
        }
    }

    fn get_best_type(card_str:&String) -> u8 {
        if !card_str.contains("J") {
            return Hand::get_type(card_str);
        }

        let mut best_type = 0;
        for c in "AKQJT98765432".chars() {
            let new_str = card_str.clone().replace("J", c.to_string().as_str());

            let type_ = Hand::get_type(&new_str);
            if type_ > best_type {
                best_type = type_;
            }
        }

        best_type
    }

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.type_ == other.type_ {
            return self.cards.cmp(&other.cards);
        }

        self.type_.cmp(&other.type_)
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cards = String::new();
        for card in &self.cards {
            cards.push(card.symbol);
        }
        write!(f, "Hand {{ cards: {}, bid: {}, type: {} }}", cards, self.bid, self.type_)
    }
}

fn read_file() -> String {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    contents
}