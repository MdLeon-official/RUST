fn make_tuple<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

fn main() {
    make_tuple("hello", 87);
    make_tuple(true, 45.4532);
    make_tuple(456, "random");
}
