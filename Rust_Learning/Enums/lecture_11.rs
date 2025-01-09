#[derive(Debug)]
enum OnlineOrderStatus {
    Ordered,
    Packed,
    Shipped,
    Delivered,
}

impl OnlineOrderStatus {
    fn check(&self) {
        match self {
            // OnlineOrderStatus::Ordered | OnlineOrderStatus::Packed => {
            //     println!("Your item is being prepped for shipment.")
            // }
            OnlineOrderStatus::Delivered => {
                println!("Your item has been delivered.")
            }
            // _ => {
            //     println!("Your item is not ready yet.")
            // }
            other_status => {
                println!("Your item is {other_status:?}")
            }
        }
    }
}

fn main() {
    OnlineOrderStatus::Delivered.check();
}
