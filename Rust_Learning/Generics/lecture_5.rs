
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

fn main() {
    let gold_chest = TreasureChest {
        captain: String::from("Luffy"),
        treaser: "Nakama",
    };
    println!("{:?}", gold_chest);
    let mut silver_chest = TreasureChest {
        captain: String::from("Kidd"),
        treaser: String::from("       Silver       "),
    };
    silver_chest.clean_treaser();
    println!("{:?}", silver_chest);

    let special_chest = TreasureChest {
        captain: String::from("WhiteBeard"),
        treaser: ["Friends", "Family", "Sake"],
    };
    println!("{}", special_chest.amount_of_treaser());
    println!("{:?}", special_chest);
}
