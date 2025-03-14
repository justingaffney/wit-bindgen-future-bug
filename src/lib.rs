wit_bindgen::generate!({
    world: "example-world"
});

use wit_bindgen::rt::async_support::{
    FutureReader,
    StreamReader
};
use crate::exports::example::pkg::example::Guest;

struct Example;

impl Guest for Example {
    fn future1() -> FutureReader<Result<Vec<String>, ()>> {
        unimplemented!()
    }

    fn future2() -> FutureReader<Result<Vec::<Vec::<u8>>, ()>> {
        unimplemented!()
    }

    fn future3() -> FutureReader<Vec::<Vec::<Vec::<u8>>>> {
        unimplemented!()
    }
    
    fn stream1() -> StreamReader<Result<Vec<String>, ()>> {
        unimplemented!()
    }

    fn stream2() -> StreamReader<Result<Vec::<Vec::<u8>>, ()>> {
        unimplemented!()
    }

    fn stream3() -> StreamReader<Vec::<Vec::<Vec::<u8>>>> {
        unimplemented!()
    }
}

export!(Example);

mod example_world;