#[derive(Debug)]
enum Meat {
    Chicken,
    Steak,
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black,
}

#[derive(Debug)]
enum RestaurantItem {
    Burrito { meat: Meat, beans: Beans },
    Bowl { meat: Meat, beans: Beans },
    VeganPlate,
}

fn main() {
    let lunch = RestaurantItem::Burrito {
        meat: Meat::Steak,
        beans: Beans::Pinto,
    };
    let dinner = RestaurantItem::Bowl {
        meat: Meat::Chicken,
        beans: Beans::Black,
    };
    let abandoned_meal = RestaurantItem::VeganPlate;
    println!("Lunch was: {:?}.", lunch);
    println!("Dinner was: {:?}.", dinner);
    println!("Nobody ate: {:?}.", abandoned_meal);
}
