wit_bindgen::generate!({
    path: "../wit",
    world: "pingpong",
});

use crate::{
    consumer::subscribe,
    messaging_types::{open_broker, Channel, EventParam},
    producer::publish,
};

use handler::Event;

struct Pingpong;

impl handler::Handler for Pingpong {
    fn on_receive(e: Event) -> Result<(), u32> {
        println!(">>> Listening...");

        let data = e.data.unwrap();
        let msg = String::from_utf8(data).unwrap();

        if !msg.is_empty() {
            let broker = open_broker("my-messaging")?;

            match msg.as_str() {
                "ping" => {
                    let ret_msg = format!(
                        "pong {:?}x",
                        match keyvalue::get("increment") {
                            Ok(curr_val) => {
                                let increment_count = curr_val.parse::<i32>().unwrap() + 1;
                                keyvalue::set("increment", &increment_count.to_string());

                                increment_count
                            }
                            Err(_) => {
                                keyvalue::set("increment", "1");

                                1
                            }
                        }
                    );

                    let new_event = EventParam {
                        data: Some(ret_msg.as_bytes()),
                        id: "123",
                        source: "rust",
                        specversion: "1.0",
                        ty: "com.my-messaing.rust.fizzbuzz",
                        datacontenttype: None,
                        dataschema: None,
                        subject: Some("pong"),
                        time: None,
                        extensions: None,
                    };

                    println!(">>> Publishing: {:#?}", new_event);
                    publish(broker, messaging_types::Channel::Topic("pong"), new_event)?;
                }
                "reset" => {
                    let new_event = EventParam {
                        data: Some("pong 0x".as_bytes()),
                        id: "123",
                        source: "rust",
                        specversion: "1.0",
                        ty: "com.my-messaing.rust.fizzbuzz",
                        datacontenttype: None,
                        dataschema: None,
                        subject: Some("reset"),
                        time: None,
                        extensions: None,
                    };
                    keyvalue::set("increment", "0");
                    publish(broker, messaging_types::Channel::Topic("ping"), new_event);
                }
                _ => {
                    println!("unknown command: {}", msg);
                }
            }
        }

        Ok(())
    }
}

export_pingpong!(Pingpong);
