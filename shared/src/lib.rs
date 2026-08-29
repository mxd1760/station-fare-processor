use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TapEventData {
    pub kiosk_id: String,
    pub card_id: String,
    pub kiosk_transaction_number: u64,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct TapEvent{
    pub event_id: String,
    pub timestamp: String,
    pub event_data: TapEventData,
}