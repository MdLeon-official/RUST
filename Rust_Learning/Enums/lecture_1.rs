
#[derive(Debug)]
enum CardSuit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

struct Card {
    rank: String,
    suit: CardSuit,
}

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;
    second_card = CardSuit::Clubs;
    println!("{:?}", second_card);

    let card_suits = [CardSuit::Hearts, CardSuit::Clubs];
    let card_suits = (CardSuit::Hearts, CardSuit::Spades);
}
