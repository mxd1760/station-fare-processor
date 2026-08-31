use rumqttc::{MqttOptions, Event, Incoming, Client, QoS};
use shared::TapEventData;
use std::time::Duration;

static ENT_ID:&str = "test-entrance-1";
static EX_ID:&str = "test-exit-1";

fn main(){
    let mut mqttoptions = MqttOptions::new("dev-automation-kiosk1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe(format!("ent-ex/{}/taps/ack",ENT_ID),QoS::AtLeastOnce).unwrap();
    client.subscribe(format!("ent-ex/{}/taps/ack",EX_ID), QoS::AtLeastOnce).unwrap();

    let event = TapEventData{
        kiosk_id: ENT_ID.to_owned(),
        card_id: "card-1".to_string(),
        kiosk_transaction_number: 1
    };

    let payload = serde_json::to_vec(&event).unwrap();
    client.publish(format!("ent-ex/{}/taps",ENT_ID),QoS::AtLeastOnce, false, payload).unwrap();

    let event = TapEventData{
        kiosk_id: EX_ID.to_owned(),
        card_id: "card-1".to_string(),
        kiosk_transaction_number: 1
    };

    let payload = serde_json::to_vec(&event).unwrap();
    client.publish(format!("ent-ex/{}/taps",EX_ID),QoS::AtLeastOnce, false, payload).unwrap();

    for notification in connection.iter(){
        match notification {
            Ok(Event::Incoming(Incoming::Publish(p))) =>{
                println!(
                    "Confirmed: {}",
                    String::from_utf8_lossy(&p.payload)
                );
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("MQTT error: {e:?}");
                break;
            },
        }
    }

}