mod Classes;
use Classes::SubAccount;

pub struct Position;
impl SubAccount for Position {
    pub fn update(&self, child: &str, id: i32, time: usize, mark_price: i32) {
        self.update(child, id, time, mark_price);
    }
}

impl Position{
    pub fn every_day_update(&self, id: i32, mark_price: i32) {
        let mut algs = data_helper::get_data("position", id); // 假設 get_data 回傳 Vec<CalPnlAlg>
        
        for alg in &mut algs {
            alg.incoming_amount += alg.trading_amount;
            alg.incoming_profit += alg.trading_profit;
            alg.incoming_pnl += mark_price * alg.trading_amount - alg.profit;
            alg.trading_pnl = 0;
            alg.trading_amount = 0;
            alg.trading_profit = 0;
            alg.optimistic_amount = 0;
            alg.optimistic_profit = 0;
            alg.optimistic_pnl = 0;
            data_helper::put_data("position", id, alg);
        }
    }

    // fn every_day_update(&self, id: i32, mark_price: i32, data_helper: &DataHelper) {
    //     let mut alg = data_helper.get_data("position", id);
    //     let n = alg.len();

    //     for i in 0..n {
    //         alg[i].incoming_amount += alg[i].trading_amount;
    //         alg[i].incoming_profit += alg[i].trading_profit;
    //         alg[i].incoming_pnl += mark_price * alg[i].trading_amount - alg[i].profit;
    //         alg[i].trading_pnl = 0;
    //         alg[i].trading_amount = 0;
    //         alg[i].trading_profit = 0;
    //         alg[i].optimistic_amount = 0;
    //         alg[i].optimistic_profit = 0;
    //         alg[i].optimistic_pnl = 0;
    //     }

    //     for i in 0..n {
    //         data_helper.put_data("position", id, alg[i]);
    //     }
    // }

}