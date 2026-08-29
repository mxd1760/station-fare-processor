use rumqttc::{Client, Event, Incoming, MqttOptions, QoS};
use shared::TapEventData;
use std::collections::HashSet;
use std::time::Duration;

fn main() {
    let mut mqttoptions = MqttOptions::new("fare-server", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("kiosks/+/taps", QoS::AtLeastOnce).unwrap();
    let mut seen_events: HashSet<String> = HashSet::new();

    for notification in connection.iter() {
        match notification {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                let event: TapEventData = match serde_json::from_slice(&p.payload) {
                    Ok(e) => e,
                    Err(e) => { eprintln!("Bad payload: {e:?}"); continue; }
                };

                let event_id = format!(
                    "{}-{}-{}", event.kiosk_id, event.card_id, event.kiosk_transaction_number
                );
                let is_new = seen_events.insert(event_id.clone());
                println!("Received tap: {event:?} (new: {is_new})");

                let ack_topic = format!("kiosks/{}/taps/ack", event.kiosk_id);
                let ack_payload = format!(
                    r#"{{"event_id":"{event_id}","status":"confirmed"}}"#
                );
                client.publish(ack_topic, QoS::AtLeastOnce, false, ack_payload).unwrap();
            }
            Ok(_) => {}
            Err(e) => { eprintln!("MQTT error: {e:?}"); break; }
        }
    }
}