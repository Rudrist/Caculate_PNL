use crate::Calculate_pnl::Order_state::OrderState;

pub struct Order {
    id: i32,
    belong_id: i32,
    state: OrderState,
    side: bool,
    amount: i32,
    price: i32,
    fee: i32,
    profit: i32,
    sell: i32,
    time_stamp: usize,
    lifo_order_id: i32, ////
}

impl Order{
    pub fn new() -> Order{
        Order{
            id: 0,
            belong_id: 0,
            state: OrderState::Pending,
            side: false,
            amount: 0,
            price: 0,
            fee: 0,
            profit: 0,
            sell: 0,
            time_stamp: 0,
            lifo_order_id: 0, ////
        }
    }
}