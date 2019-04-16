#![no_main]
extern crate kepler_core;
#[macro_use]
extern crate libfuzzer_sys;

use kepler_core::core::Block;
use kepler_core::ser;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<Block, ser::Error> = ser::deserialize(&mut d);
});
