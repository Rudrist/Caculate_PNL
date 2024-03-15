pub struct Pnl_Alg {
    incoming_pnl: i32,
    incoming_amount: i32,
    incoming_profit: i32,
    trading_pnl: i32,
    trading_amount: i32,
    trading_profit: i32,
    optimistic_pnl: i32,
    optimistic_amount: i32,
    optimistic_profit: i32,
    fifo_order_id: i32,
    realized_amount: i32,
    realized_profit: i32,
    realized_pnl: i32,
    unrealized_amount: i32,
    unrealized_profit: i32,
    unrealized_pnl: i32,
    total_pnl: i32,
    net_pnl: i32,
}

impl Pnl_Alg{
    pub fn new() -> Pnl_Alg{
        Pnl_Alg{
            incoming_pnl: 0,
            incoming_amount: 0,
            incoming_profit: 0,
            trading_pnl: 0,
            trading_amount: 0,
            trading_profit: 0,
            optimistic_pnl: 0,
            optimistic_amount: 0,
            optimistic_profit: 0,
            fifo_order_id: 0,
            realized_amount: 0,
            realized_profit: 0,
            realized_pnl: 0,
            unrealized_amount: 0,
            unrealized_profit: 0,
            unrealized_pnl: 0,
            total_pnl: 0,
            net_pnl: 0,
        }
    }
}