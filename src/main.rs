#[derive(Debug)]

struct Desk {
    cards : Vec<String>,
}

impl Desk {
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
    //inicializacion de struct desk previo
    /* let desk = Desk {cards};
    return desk; */

    //utilizamos implicit return
    Desk { cards }
    
    }
}

fn main() {

    let desk = Desk::new();
    
    println!("Your cards are: {:#?}", desk);
}