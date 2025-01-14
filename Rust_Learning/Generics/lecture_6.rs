#[derive(Debug)]
struct TreasureChest<T> {
    captain: String,
    treaser: T,
}

impl TreasureChest<String> {
    fn clean_treaser(&mut self) {
        self.treaser = self.treaser.trim().to_string()
    }
}

impl TreasureChest<[&str; 3]> {
    fn amount_of_treaser(&self) -> usize {
        self.treaser.len()
    }
}

impl<T> TreasureChest<T> {
    fn capital_captain(&self) -> String {
        self.captain.to_uppercase()
    }
}

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Luffy"),
        treaser: "Nakama",
    };
    println!("{:?}", gold_chest.capital_captain());
    let mut silver_chest = TreasureChest {
        captain: String::from("Kidd"),
        treaser: String::from("       Silver       "),
    };
    silver_chest.clean_treaser();
    println!("{:?}", silver_chest.capital_captain());

    let special_chest = TreasureChest {
        captain: String::from("WhiteBeard"),
        treaser: ["Friends", "Family", "Sake"],
    };
    println!("{}", special_chest.amount_of_treaser());
    println!("{:?}", special_chest.capital_captain());
}
