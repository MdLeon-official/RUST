#[derive(Debug)]

struct OnePiece {
    title: String,
    release_year: u32,
}

impl OnePiece {
    fn display_all_ep(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
    }
}

fn main() {
    let episode = OnePiece {
        title: String::from("Gear 5th"),
        release_year: 2022,
    };
    episode.display_all_ep();
}
