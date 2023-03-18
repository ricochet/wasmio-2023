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
        // let my_pub = Pub::open("my-messaging")?;
        // let my_sub = Sub::open("my-messaging")?;
        // let ping_pong_sub_tok = my_sub.subscribe("ping")?;
        // let keyvalue = Keyvalue::open("my-container")?;
        println!(">>> Listening...");

        // todo on receive
        // let msg = my_sub.receive(&ping_pong_sub_tok)?;
        // let msg;

        // if !msg.is_empty() {
        //     let msg_s = String::from_utf8(msg)?;
        //     match msg_s.as_str() {
        //         "ping" => {
        //             let ret_msg = format!(
        //                 "pong {:?}x",
        //                 match keyvalue.get("increment") {
        //                     Ok(curr_val) => {
        //                         let curr_val_s = String::from_utf8(curr_val)?;
        //                         let increment_count = curr_val_s.as_str().parse::<u32>()? + 1;
        //                         keyvalue
        //                             .set("increment", increment_count.to_string().as_bytes())?;

        //                         increment_count
        //                     }
        //                     Err(_) => {
        //                         keyvalue.set("increment", "1".as_bytes())?;

        //                         1
        //                     }
        //                 }
        //             );

        //             publish(ret_msg.as_bytes(), "pong")?;
        //         }
        //         "reset" => {
        //             keyvalue.set("increment", "0".as_bytes())?;
        //             publish("pong 0x".as_bytes(), "pong");
        //         }
        //         _ => {
        //             println!("unknown command: {}", msg_s.as_str());
        //         }
        //     }
        // }

        println!("made it");

        Ok(())
    }
}

export_pingpong!(Pingpong);
