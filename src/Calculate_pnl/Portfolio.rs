use crate::Calculate_pnl::Pnl_Alg::Pnl_Alg;
pub struct Portfolio{
    Alg: Pnl_Alg,
}
impl Portfolio {
    pub fn new() -> Portfolio{
        Portfolio {Alg: Pnl_Alg::new() }
    }
    pub fn update(&self) {
        //self.update(child, id, time, mark_price);
        println!("test output: Portfolio");
    }    
}