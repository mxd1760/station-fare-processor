use serde::{Serialize, Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone, PartialEq, Eq)]
pub struct TapEventData {
    pub kiosk_id: String,
    pub card_id: String,
    pub kiosk_transaction_number: u64,
}

#[derive(Serialize,Deserialize,Debug,Clone, PartialEq, Eq)]
pub struct TapEvent{
    pub event_id: String,
    pub timestamp: String,
    pub event_data: TapEventData,
}

impl TapEvent{

    fn new(timestamp:&str,event_data:&TapEventData) -> Self{
        let event_id = format!{"{}-{}-{}-{}",event_data.kiosk_id,event_data.card_id,event_data.kiosk_transaction_number,timestamp};
        Self { event_id, timestamp: timestamp.to_owned(), event_data:event_data.clone()}
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tap_event_data_serialization_and_deserialization(){
        let kid = "test_kiosk_1";
        let cid = "test_card_1";
        let original = TapEventData{
            kiosk_id:kid.to_owned(),
            card_id:cid.to_owned(),
            kiosk_transaction_number:1
        };

        let encoded = serde_json::to_string(&original).unwrap();
        let decoded: TapEventData = serde_json::from_str(&encoded).unwrap();

        assert_eq!(original,decoded);
    }

    #[test]
    fn tap_event_serialization_and_deserialization(){
        let kid = "test_kiosk_1";
        let cid = "test_card_1";
        let original_ted = TapEventData{
            kiosk_id:kid.to_owned(),
            card_id:cid.to_owned(),
            kiosk_transaction_number:1
        };
        let current_date_time_str = "2026-09-01 12:30:00";

        let original_full = TapEvent::new(current_date_time_str,&original_ted);


        let encoded = serde_json::to_string(&original_full).unwrap();
        let decoded: TapEvent = serde_json::from_str(&encoded).unwrap();

        assert_eq!(original_full,decoded);
        assert_eq!(original_ted,decoded.event_data)
    }

    #[test]
    fn generated_ids(){
        let kid = "test_kiosk_1";
        let cid = "test_card_1";
        let original_ted = TapEventData{
            kiosk_id:kid.to_owned(),
            card_id:cid.to_owned(),
            kiosk_transaction_number:1
        };
        let current_date_time_str = "2026-09-01 12:30:00";

        let original_full = TapEvent::new(current_date_time_str,&original_ted);

        assert_eq!(original_full.event_id,"test_kiosk_1-test_card_1-1-2026-09-01 12:30:00");
    }
}