#[derive(Debug)]

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal(String, String),
}

fn main() {
    let mut my_payment_method = PaymentMethodType::CreditCard((String::from("14-134-141")));

    my_payment_method = PaymentMethodType::PayPal(
        String::from("me@gmail.com"),
        String::from("thisismypassword"),
    );
    println!("{:?}", my_payment_method);
}
