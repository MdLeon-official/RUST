struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    // Method to compare durations
    fn is_longer_than(&self, other: &TaylorSwiftSong) -> bool {
        self.duration_secs > other.duration_secs
    }
}

fn main() {
    // Create two song instances
    let blank_space = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    let all_too_well = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2012,
        duration_secs: 327,
    };

    // Compare durations
    if blank_space.is_longer_than(&all_too_well) {
        println!("{} is longer than {}", blank_space.title, all_too_well.title);
    } else {
        println!(
            "{} is shorter than or equal to {}",
            blank_space.title, all_too_well.title
        );
    }
}
