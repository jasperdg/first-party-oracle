use near_sdk::{env, StorageUsage, Balance, AccountId, Promise};

pub fn refund_storage(initial_storage: StorageUsage, sender_id: AccountId) {
    let current_storage = env::storage_usage();
    let attached_deposit = env::attached_deposit();
    let refund_amount = if current_storage > initial_storage {
        let required_deposit =
            Balance::from(current_storage - initial_storage) * env::storage_byte_cost();
        assert!(
            required_deposit <= attached_deposit,
            "The required attached deposit is {}, but the given attached deposit is is {}",
            required_deposit,
            attached_deposit,
        );
        attached_deposit - required_deposit
    } else {
        attached_deposit + Balance::from(initial_storage - current_storage) * env::storage_byte_cost()
    };
    if refund_amount > 0 {
        Promise::new(sender_id).transfer(refund_amount);
    }
}

pub fn round(n: f64, precision: u32) -> f64 {
    (n * 10_u32.pow(precision) as f64).round() / 10_i32.pow(precision) as f64
}

pub fn precision(x: f64) -> Option<u32> {
    for digits in 0..std::f64::DIGITS {
        if round(x, digits) == x {
            return Some(digits);
        }
    }
    None
}