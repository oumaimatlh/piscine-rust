use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Suit {
    Heart,
    Spade,
    Club, 
    Diamond,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Rank {
    Ace, 
    King, 
    Jack, 
    Queen,
    Number(u8)
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let n = rng.gen_range(0..=3); 
        [Suit::Heart, Suit::Diamond, Suit::Spade, Suit::Club][n]
    }

    pub fn translate(value: u8) -> Suit {
        if value == 1 {return Suit::Heart};
        if value == 2 {return Suit::Diamond};
        if value == 3 {return Suit::Spade};
        if value == 4 {return Suit::Club};
        Suit::Spade

    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let rank = rng.gen_range(0..=4);

        let n = rng.gen_range(2..=10);
        [Rank::Ace, Rank::King, Rank::Queen, Rank::Jack, Rank::Number(n)][rank]
    }

    pub fn translate(value: u8) -> Rank {
        if value == 1 {return Rank::Ace};
        if value == 2 {return Rank::Number(2)};
            
        if value == 3 {return Rank::Number(3)};
        if value == 4 {return Rank::Number(4)};
        if value == 5 {return Rank::Number(5)};
        if value == 6 {return Rank::Number(6)};
        if value == 7 {return Rank::Number(7)};
        if value == 8 {return Rank::Number(8)};
        if value == 9 {return Rank::Number(9)};

        if value == 11 {return Rank::Jack};
        if value == 12 {return Rank::Queen};
        if value == 13 {return Rank::King};
        Rank::Ace
    }
}
#[derive(Debug, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    //Suit::Heart or ... 
    //Rank::King wla number ....

    if card.suit == Suit::Spade  && card.rank == Rank::Ace {
        return true
    }
    false
}
