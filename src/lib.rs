wit_bindgen::generate!({
    world: "example-world"
});

use wit_bindgen::rt::async_support::FutureReader;
use crate::exports::example::pkg::example::Guest;

struct Example;

impl Guest for Example {
    fn test() -> FutureReader<Result<Vec<String>, ()>> {
        todo!()
    }
}

export!(Example);