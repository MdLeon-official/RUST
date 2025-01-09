enum Milk {
    Lowfat(i32),
    Whole,
}

impl Milk {
    fn drink(self) {
        match self {
            Milk::Lowfat(2) => {
                println!("Delicious, 2% milk is my favourite!");
            }
            Milk::Lowfat(percent) => {
                println!("You've got the lowfat {percent} percent version!")
            }
            Milk::Whole => {
                println!("You've got the whole milk!");
            }
        }
    }
}

fn main() {
    Milk::Lowfat(2).drink();
    Milk::Lowfat(1).drink();
    Milk::Whole.drink();
}
