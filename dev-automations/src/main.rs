use rumqttc::{MqttOptions, Event, Incoming, Client, QoS};
use shared::TapEventData;
use std::time::Duration;

fn main(){
    let mut mqttoptions = MqttOptions::new("dev-automation-kiosk1", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("kiosks/entrance-1/taps/ack",QoS::AtLeastOnce).unwrap();

    let event = TapEventData{
        kiosk_id: "entrance-1".to_string(),
        card_id: "card-1".to_string(),
        kiosk_transaction_number: 1
    };

    let payload = serde_json::to_vec(&event).unwrap();
    client.publish("kiosks/entrance-1/taps",QoS::AtLeastOnce, false, payload).unwrap();
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