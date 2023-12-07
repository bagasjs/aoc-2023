use std::env;
use std::fs;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Unknown,
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAkind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

struct Hand {
    cards: [Card; 5],
    kind: HandKind,
    bid: u64,
}

impl Hand {
    fn new(cards: &str, bid: u64) -> Result<Self, String> {
        if cards.len() != 5 {
            return Err("Allowed card for a hand is only 5".to_string());
        }

        let mut result = Self { bid, kind: HandKind::HighCard, cards: [Card::Unknown, Card::Unknown, Card::Unknown, Card::Unknown, Card::Unknown] };
        let cards: Vec<char> = cards.chars().collect();

        let mut joker_count = 0;
        for i in 0..5 {
            let current_card = match cards[i] {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                'J' => Card::Joker,
                _ => Card::Unknown,
            };

            if current_card == Card::Joker {
                joker_count += 1;
            } else {
                let mut same_card_counter = 0;
                for j in 0..i {
                    if current_card == result.cards[j] {
                        same_card_counter += 1;
                    }
                }

                result.kind = match same_card_counter {
                    0 => result.kind,
                    1 => match result.kind {
                        HandKind::HighCard => HandKind::OnePair,
                        HandKind::OnePair => HandKind::TwoPair,
                        HandKind::ThreeOfAkind => HandKind::FullHouse,
                        _ => {
                            return Err(format!("Unexpected kind transition from {:?} with {same_card_counter} matching cards", result.kind))
                        }
                    },
                    2 => match result.kind {
                        HandKind::OnePair => HandKind::ThreeOfAkind,
                        HandKind::TwoPair => HandKind::FullHouse,
                        _ => {
                            return Err(format!("Unexpected kind transition from {:?} with {same_card_counter} matching cards", result.kind))
                        },
                    },
                    3 => match result.kind {
                        HandKind::ThreeOfAkind => HandKind::FourOfAKind,
                        _ => {
                            return Err(format!("Unexpected kind transition from {:?} with {same_card_counter} matching cards", result.kind))
                        }
                    },
                    4 => match result.kind {
                        HandKind::FourOfAKind => HandKind::FiveOfAKind,
                        _ => {
                            return Err(format!("Unexpected kind transition from {:?} with {same_card_counter} matching cards", result.kind))
                        }
                    }
                    _ => return Err("Unreachable error".to_string())
                };
            }
            result.cards[i] = current_card;
        }

        result.kind = match joker_count {
            0 => result.kind,
            1 => match result.kind {
                HandKind::HighCard => HandKind::OnePair,
                HandKind::OnePair => HandKind::ThreeOfAkind,
                HandKind::ThreeOfAkind => HandKind::FourOfAKind,
                HandKind::FourOfAKind => HandKind::FiveOfAKind,
                HandKind::TwoPair => HandKind::FullHouse,
                _ => {
                    return Err(format!("Unexpected kind transition from {:?} with {joker_count} jokers", result.kind))
                },
            },
            2 => match result.kind {
                HandKind::HighCard => HandKind::ThreeOfAkind,
                HandKind::OnePair => HandKind::FourOfAKind,
                HandKind::ThreeOfAkind => HandKind::FiveOfAKind,
                _ => {
                    return Err(format!("Unexpected kind transition from {:?} with {joker_count} jokers", result.kind))
                },
            },
            3 => match result.kind {
                HandKind::HighCard => HandKind::FourOfAKind,
                HandKind::OnePair => HandKind::FiveOfAKind,
                _ => {
                    return Err(format!("Unexpected kind transition from {:?} with {joker_count} jokers", result.kind))
                }
            },
            4 => match result.kind {
                HandKind::HighCard => HandKind::FiveOfAKind,
                _ => {
                    return Err(format!("Unexpected kind transition from {:?} with {joker_count} jokers", result.kind))
                }
            }
            5 => HandKind::FiveOfAKind,
            _ => return Err("Unreachable error".to_string())
        };

        Ok(result)
    }
}

fn cmp_hands(a: &Hand, b: &Hand) -> std::cmp::Ordering {
    match a.kind.cmp(&b.kind) {
        std::cmp::Ordering::Equal => {
            for i in 0..a.cards.len() {
                match a.cards[i].cmp(&b.cards[i]) {
                    std::cmp::Ordering::Equal => continue,
                    std::cmp::Ordering::Greater => return std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Less => return std::cmp::Ordering::Less,
                }
            }
            return std::cmp::Ordering::Equal;
        },
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    }
}

fn solve_file(file_path: &str) -> Result<u64, String> {
    let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    let mut hands: Vec<Hand> = Vec::new();
    for line in file_content.lines() {
        let mut line_it = line.split_whitespace().into_iter();
        let cards = line_it.next().ok_or("Failed to parse cards")?;
        let bid: u64 = line_it.next()
            .ok_or("Failed to parse the bid")?
            .parse()
            .map_err(|_| "Failed to parse bid to number")?;
        let hand = Hand::new(cards, bid)?;
        hands.push(hand);
    }
    hands.sort_by(|a, b| cmp_hands(a, b));
    let mut result = 0;
    for i in 0..hands.len() {
        println!("{:?} {:?}", hands[i].cards, hands[i].kind);
        result += hands[i].bid * (i + 1) as u64;
    }
    return Ok(result);
}

fn main() {
    let file_path = env::args().skip(1).next().expect("Please provide an input file path");

    match solve_file(&file_path) {
        Ok(result) => {
            println!("Result: {result}");
        },
        Err(error_message) => eprintln!("ERROR: {}", error_message),
    }
}
