use crate::Calculate_pnl::Pnl_Alg::Pnl_Alg;

pub struct Quotation{
    Alg: Pnl_Alg,
}

impl Quotation {
    pub fn new() -> Quotation{
        Quotation {Alg: Pnl_Alg::new() }
    }
    pub fn update(&self) {
        //self.update(child, id, time, mark_price);
        println!("test output: quotation");
    }    
}