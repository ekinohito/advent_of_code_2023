#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Card(u8);

impl Card {
    pub const JOKER: Self = Self(0);
    pub fn from_char(ch: char) -> Result<Self, &'static str> {
        Ok(Self(match ch {
            '0' => return Ok(Self::JOKER),
            '2'..='9' => ch.to_digit(10).unwrap() as u8,
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => return Err("Bad char"),
        }))
    }

    pub fn from_char_use_joker(ch: char) -> Result<Self, &'static str> {
        if ch == 'J' {
            return Ok(Self::JOKER);
        }
        Self::from_char(ch)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Kind {
    pub fn from_cards(cards: &[Card; 5]) -> Self {
        let sorted_cards = {
            let mut temp = *cards;
            temp.sort();
            temp
        };
        let mut multiset = Vec::with_capacity(5);
        let mut last_card = sorted_cards[0];
        multiset.push(0);
        for card in sorted_cards {
            if card == last_card {
                *multiset.last_mut().unwrap() += 1;
            } else {
                last_card = card;
                multiset.push(1);
            }
        }

        if sorted_cards[0] == Card::JOKER && multiset.len() > 1 {
            let jokers = multiset[0];
            multiset.rotate_left(1);
            multiset.pop();
            *multiset.iter_mut().max().unwrap() += jokers;
        }

        match &multiset[..] {
            [_] => Kind::FiveOfAKind,
            [1 | 4, _] => Kind::FourOfAKind,
            [_, _] => Kind::FullHouse,
            [1 | 2, 1 | 2, 1 | 2] => Kind::TwoPair,
            [_, _, _] => Kind::ThreeOfAKind,
            [_, _, _, _] => Kind::OnePair,
            _ => Kind::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Hand(Kind, [Card; 5], pub u64);

impl Hand {
    pub fn from_str(from: &str, bid: u64, use_joker: bool) -> Result<Self, &'static str> {
        let cards: Result<Vec<_>, _> = from
            .chars()
            .map(if use_joker {
                Card::from_char_use_joker
            } else {
                Card::from_char
            })
            .take(5)
            .collect();
        let cards = cards?;
        let cards: Result<&[Card; 5], _> = cards.as_slice().try_into();
        let cards = cards.map_err(|_| "Not enough characters")?;
        let kind = Kind::from_cards(cards);
        Ok(Self(kind, *cards, bid))
    }
}

#[test]
fn test_hand() {
    assert_eq!(
        Hand::from_str("QQQQQ", 100, false),
        Ok(Hand(
            Kind::FiveOfAKind,
            [Card(12), Card(12), Card(12), Card(12), Card(12)],
            100
        ))
    );
    assert_eq!(
        Hand::from_str("AA8AA", 150, false),
        Ok(Hand(
            Kind::FourOfAKind,
            [Card(14), Card(14), Card(8), Card(14), Card(14)],
            150
        ))
    );
    assert_eq!(
        Hand::from_str("56665", 200, false),
        Ok(Hand(
            Kind::FullHouse,
            [Card(5), Card(6), Card(6), Card(6), Card(5)],
            200
        ))
    );
    assert_eq!(
        Hand::from_str("23432", 300, false),
        Ok(Hand(
            Kind::TwoPair,
            [Card(2), Card(3), Card(4), Card(3), Card(2)],
            300
        ))
    );
}
