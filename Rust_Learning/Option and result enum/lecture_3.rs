fn main() {
    let musical_instrument = [
        String::from("Guitar"),
        String::from("Drums"),
        String::from("Harmonica"),
    ];

    let harmonica: Option<&String> = musical_instrument.get(2);
    println!("{:?}", harmonica);
    let valid_harmonica = harmonica.unwrap();
    println!("Unwrap method: {valid_harmonica}");
    let valid_harmonica = harmonica.expect("Unable to retrieve data.");
    println!("Expect method: {valid_harmonica}");

    let invalid_instrument = musical_instrument.get(100);
    println!("{:?}", invalid_instrument);
    // invalid_instrument.unwrap(); //gives runtime error
    // invalid_instrument.expect("Unable to retrieve data."); //gives runtime error with this message
}
