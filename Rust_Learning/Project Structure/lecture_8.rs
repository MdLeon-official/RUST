// ===>  INSIDE INVENTORY.RS
//pub const FLOOR_SPACE: i32 = 10000;
// pub const MANAGER: &str = "Mine Inventory";

// #[derive(Debug)]
// pub enum ProductCategory {
//     Ladder,
//     Hammer,
// }

// #[derive(Debug)]
// pub struct Item {
//     pub name: String,
//     pub category: ProductCategory,
//     pub quantity: u32,
// }

// pub fn talk_to_manager() {
//     println!("Hey, {MANAGER}, how's your coffee")
// }



// ===>  INSIDE MOD.RS LOCATED IN THE ORDERS FOLDER
//pub const MANAGER: &str = "Mine orders";





mod inventory;
mod orders;

fn main() {
    println!(
        "Our managers are {} and {}. We have {} square feet of floor space",
        inventory::MANAGER,
        orders::MANAGER,
        inventory::FLOOR_SPACE
    );

    inventory::talk_to_manager();

    let favourite_category = inventory::ProductCategory::Hammer;
    println!("My favourite category of item is {favourite_category:?}");

    let tall_ladder = inventory::Item {
        name: String::from("Ladder-o-matic 2000"),
        category: favourite_category,
        quantity: 100,
    };
    println!("{:#?}", tall_ladder);
}
