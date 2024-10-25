#[derive(Debug)]

struct Deck {
    cards : Vec<String>,
}

impl Deck {
    //Associated function en Deck
    fn new() -> Self {

        let suits = ["Hearts","Spades","Diamonds"];

    let values= ["one", "two", "three"];

    let mut cards = vec![];

    for suit in suits {

        for value in values{

            let card = format!("{} of {} ", value, suit);
            
            cards.push(card);
        }
    }
    //inicializacion de struct deck previo
    /* let deck = Deck {cards};
    return deck; */

    //utilizamos implicit return
    Deck { cards }
    
    }
}

fn main() {
    //inicializacióń de una Associated function 
    let deck = Deck::new();
    
    println!("Your cards are: {:#?}", deck);
}
