use rumqttc::{MqttOptions, Event, Incoming, Client, QoS};
use shared::TapEventData;
use std::{path::Path, time::Duration};
use clap::{Command, arg, command};


static ENT_ID:&str = "test-entrance-1";
static EX_ID:&str = "test-exit-1";

enum FareProcessingMode{
    Distance,
    Segment,
    Zone,
    Pair
}

impl FareProcessingMode{
    fn from_str(mode:&str)->Self{
        match mode{
            "Segment"=>Self::Segment,
            "Zone"=>Self::Zone,
            "Pair"=>Self::Pair,
            _=>Self::Distance
        }
    }
}

fn main(){
    let matches = command!()
    .propagate_version(true)
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
        Command::new("test-mqtt")
        .about("starts mqtt client loop and publishes 2 commands")
    ).subcommand(
        Command::new("solve-fares")
        .about("read in map data with given ruleset to generate fare file for use on server")
        .arg(arg!([file]).required(true))
        .arg(arg!([mode]))
    ).get_matches();

    match matches.subcommand(){
        Some(("test-mqtt",_sub_matches))=>{
            test_mqtt()
        },
        Some(("solve-fares",sub_matches))=>{
            let path = Path::new(sub_matches.get_one::<String>("file").unwrap());
            let mode = FareProcessingMode::from_str(match sub_matches.get_one::<String>("mode") {
                Some(v)=>v,
                _=>""
            });
            solve_fares(path,mode)
        }
        _ => todo!(),
    }
}

fn solve_fares(file:&Path,mode:FareProcessingMode){
    match mode{
        FareProcessingMode::Distance=>{
            println!("parsing based on distance rules");

        },
        _=>{
            println!("oops, you provided an invalid fare processing mode");
        }
    }
}


fn test_mqtt(){
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