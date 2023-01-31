
#[derive(PartialEq, PartialOrd)]
enum Rank {
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace
}

enum Suit {
    Heart, Spade, Club, Diamond
}

struct Card(Rank, Suit);

struct Hand(Vec<Card>);

impl Ord for Card {
    fn cmp(&self, y:&Self) -> Ordering {

    }
}

// implement Ord to sort the cards in each hand
// once the hand is sorted it is trivial to find all hand ranks in a single pass

enum HandRank {
    HighCard(Card),
    OnePair(Card),
    TwoPair(Card, Card),
    ThreeOfKind(Card),
    Straight(Card),
    Flush(Suit),
    FullHouse(Card, Card),
    FourOfKind(Card),
    StraightFlush(Suit, Card),
    RoyalFlush(Suit)
}

fn high_card(h:Hand) -> Card {

}

fn pairs(h:Hand) -> Vec<Card> {

}

fn triples(h:Hand) -> Vec<Card> {

}

fn rank_seqs(h:Hand>) -> Vec<Vec<Card>> {

}


fn main() {

}
