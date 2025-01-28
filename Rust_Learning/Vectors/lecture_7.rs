fn main() {
    let mut season: Vec<&str> = Vec::with_capacity(4);
    println!("Length: {}, Capacity: {}", season.len(), season.capacity());

    season.push("Summer");
    season.push("Rain");
    season.push("Winter");
    season.push("Spring");
    println!("Length: {}, Capacity: {}", season.len(), season.capacity());

    season.push("Fall");
    println!("Length: {}, Capacity: {}", season.len(), season.capacity());
}
