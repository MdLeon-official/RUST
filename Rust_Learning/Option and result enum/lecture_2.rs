fn main() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Harmonica"),
    ];

    let harmonica: Option<&String> = musical_instrument.get(2);
    println!("{:?}", harmonica);

    let invalid_instrument = musical_instrument.get(100);
    println!("{:?}", invalid_instrument);
}
