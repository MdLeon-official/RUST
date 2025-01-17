fn main() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Harmonica"),
    ];

    let harmonica: Option<&String> = musical_instrument.get(2);
    // match harmonica {
    //     Option::Some(instrument) => println!("Playing the {instrument}."),
    //     Option::None => println!("Singing with my voice."),
    // }
    play(harmonica);
    println!("{:?}", harmonica);

    let invalid_instrument = musical_instrument.get(100);
    // match invalid_instrument {
    //     Option::Some(instrument) => println!("Playing the {instrument}."),
    //     Option::None => println!("Singing with my voice."),
    // }
    play(invalid_instrument);
}

fn play(instrument_option: Option<&String>) {
    match instrument_option {
        Option::Some(instrument) => println!("Playing the {instrument}."),
        Option::None => println!("Singing with my voice."),
    }
}
