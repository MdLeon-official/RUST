#[derive(Debug)]

enum PaymentMethodType {
    CreditCard(String),
    DebitCard(String),
    PayPal,
}

fn main() {
    let visa = PaymentMethodType::CreditCard(String::from("1232-5464"));
    let mastercard = PaymentMethodType::DebitCard(String::from("1232-546-3254-3423"));

    println!("VISA: {:?} and MASTERCARD: {:?}", visa, mastercard)
}

