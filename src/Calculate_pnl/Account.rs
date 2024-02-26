mod Classes;
use Classes::SubAccount;

pub struct Account;
impl SubAccount for Account {
    pub fn update(&self, child: &str, id: i32, time: usize, mark_price: i32) {
        self.update(child, id, time, mark_price);
    }
}