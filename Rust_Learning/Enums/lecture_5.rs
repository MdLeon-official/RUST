#[derive(Debug)]

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal { username: String, password: String },
    Cash,
}

fn main() {
    let paypal = PaymentMethodType::PayPal {
        username: String::from("me@gmail.com"),
        password: String::from("thisismypassword"),
    };

    println!("{:?}", paypal);
}
