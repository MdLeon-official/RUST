fn main() {
    let ok: Result<i8, &str> = Ok(8);
    println!("{:?}", ok);

    let disaster: Result<i32, &str> = Err("Something went wrong");
    println!("{:?}", disaster);
}
