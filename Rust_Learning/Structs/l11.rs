#[derive(Debug)]

struct OnePiece {
    title: String,
    release_year: u32,
    duration: u32,
}

impl OnePiece {
    fn display_all_ep(self) {
        println!("Title: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration: {}", self.duration);
    }

    fn OnePieceMovie(mut self) {
        self.duration = 100 + self.duration;
        println!("The Z movie length is {}", self.duration);
    }
}

fn main() {
    let episode = OnePiece {
        title: String::from("Gear 5th"),
        release_year: 2022,
        duration: 23,
    };

    // episode.display_all_ep();
    episode.OnePieceMovie();
}
