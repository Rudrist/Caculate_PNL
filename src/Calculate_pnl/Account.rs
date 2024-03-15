use crate::Calculate_pnl::Pnl_Alg::Pnl_Alg;

pub struct Account{
    Alg: Pnl_Alg,
}
impl Account {
    pub fn new() -> Account{
        Account {Alg: Pnl_Alg::new() }
    }
    pub fn update(&self) {
        //self.update(child, id, time, mark_price);
        println!("test output: Account");
    }   
}