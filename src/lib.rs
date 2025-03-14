wit_bindgen::generate!({
    world: "example-world"
});

use wit_bindgen::rt::async_support::FutureReader;
use crate::exports::example::pkg::example::Guest;

struct Example;

impl Guest for Example {
    fn test1() -> FutureReader<Result<Vec<String>, ()>> {
        unimplemented!()
    }

    fn test2() -> FutureReader<Result<Vec::<Vec::<u8>>, ()>> {
        unimplemented!()
    }

    fn test3() -> FutureReader<Vec::<Vec::<Vec::<u8>>>> {
        unimplemented!()
    }
}

export!(Example);