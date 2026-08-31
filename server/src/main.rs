use rumqttc::{AsyncClient, Event, Incoming, MqttOptions, Publish, QoS};
use shared::TapEventData;
use tokio::sync::Mutex;
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;

struct ServerState{
    client:AsyncClient,
    seen_events:HashSet<String>
}

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("fare-server", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    
    let (client, mut event_loop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("ent-ex/+/taps", QoS::AtLeastOnce).await.unwrap();

    let seen_events: HashSet<String> = HashSet::new();

    let server_state = Arc::new(Mutex::new(ServerState{client,seen_events}));
    loop{
        match event_loop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                tokio::spawn(handle_publish(Arc::clone(&server_state),p));
            }
            Ok(_) => {}
            Err(e) => { eprintln!("MQTT error: {e:?}"); break; }
        }
    }

}

async fn handle_publish(state:Arc<Mutex<ServerState>>,p:Publish)->Result<String,String>{
    let event: TapEventData = match serde_json::from_slice(&p.payload) {
        Ok(e) => e,
        Err(e) => { 
            let s = format!("Bad payload: {e:?}");
            eprintln!("{s:?}"); 
            return Err(s); 
        }
    };
    let event_id = format!(
        "{}-{}-{}", event.kiosk_id, event.card_id, event.kiosk_transaction_number
    );
    let ack_topic = format!("ent-ex/{}/taps/ack", event.kiosk_id);
    let ack_payload = format!(
        r#"{{"event_id":"{event_id}","status":"confirmed"}}"#
    );

    let mut state = state.lock().await;
    let is_new = state.seen_events.insert(event_id.clone());
    println!("Received tap: {event:?} (new: {is_new})");
    state.client.publish(ack_topic, QoS::AtLeastOnce, false, ack_payload).await.unwrap();
    Ok(event_id)
}