#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treaser: T,
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Luffy"),
        treaser: "Nakama",
    };
    println!("{:?}", gold_chest);
  
    let silver_chest = TreasureChest {
        captain: String::from("Kidd"),
        treaser: String::from("Silver"),
    };
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("WhiteBeard"),
        treaser: ["Friends", "Family", "Sake"],
    };
    println!("{:?}", special_chest);
}
