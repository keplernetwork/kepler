#![no_main]
#[macro_use]
extern crate libfuzzer_sys;
extern crate kepler_core;
extern crate kepler_p2p;

use kepler_core::ser;
use kepler_p2p::types::PeerAddr;

fuzz_target!(|data: &[u8]| {
	let mut d = data.clone();
	let _t: Result<PeerAddr, ser::Error> = ser::deserialize(&mut d);
});
