#![cfg(target_arch = "wasm32")]

wit_bindgen::generate!({
    world: "actor",
    path: "../../wit",
});

struct Actor;

impl actor::Actor for Actor {
    fn guest_call(operation: String, payload: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
        assert_eq!(payload, None);
        assert_eq!(operation, "Pingpong.Pingpong");
        Ok(Some(pingpong_guest::pingpong().into()))
    }
}

export_guest!(Actor);
