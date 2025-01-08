enum LaundryCycle {
    Cold,
    Hot { temperature: u32 },
    Delicate(String),
}

impl LaundryCycle {
    fn wash_laundry(&self) {
        match self {
            LaundryCycle::Cold => {
                println!("Running the laundry with cold temperature.")
            }
            LaundryCycle::Hot { temperature } => {
                println!("Running a laundry with a temperature of {temperature}.")
            }
            LaundryCycle::Delicate(fabric_type) => {
                println!("Running the laundry with a delicate cycle for {fabric_type}.")
            }
        }
    }
}

fn main() {
    (LaundryCycle::Cold).wash_laundry();
    (LaundryCycle::Hot { temperature: 85 }).wash_laundry();
    (LaundryCycle::Delicate(String::from("silk"))).wash_laundry();
}
