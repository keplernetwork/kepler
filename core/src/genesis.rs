// Copyright 2018 The Kepler Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Definition of the genesis block. Placeholder for now.

// required for genesis replacement
//! #![allow(unused_imports)]

#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]

use chrono::prelude::{TimeZone, Utc};

use crate::core;
use crate::global;
use crate::pow::{Difficulty, Proof, ProofOfWork};
use crate::util;
use crate::util::secp::constants::SINGLE_BULLET_PROOF_SIZE;
use crate::util::secp::pedersen::{Commitment, RangeProof};
use crate::util::secp::Signature;

use crate::core::hash::Hash;
use crate::keychain::BlindingFactor;

/// Genesis block definition for development networks. The proof of work size
/// is small enough to mine it on the fly, so it does not contain its own
/// proof of work solution. Can also be easily mutated for different tests.
pub fn genesis_dev() -> core::Block {
	core::Block::with_header(core::BlockHeader {
		height: 0,
		// previous: core::hash::Hash([0xff; 32]),
		timestamp: Utc.ymd(1997, 8, 4).and_hms(0, 0, 0),
		pow: ProofOfWork {
			nonce: global::get_genesis_nonce(),
			..Default::default()
		},
		..Default::default()
	})
}

/// Floonet genesis block
pub fn genesis_floo() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(2019, 2, 19).and_hms(15, 20, 34),
		prev_root: Hash::from_hex(
			"00000000000000000007f18560d29503729ee37f1365f42fdacc304176ceb759",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"90ffe71cd746676725e78557fd514f9005eb89a3c9a28c37eedab15db33f1fab",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"0ec636bcbcd9e5384b959cbd5e7d8913ab9e40efe0d67a927e5be27ca0918592",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"ff78ec0053afe57adf4e206e76fb6ca1e9b35f6f8091c203e39d1e3b42d7c18c",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
			nonce: 3,
			proof: Proof {
				nonces: vec![
					1602431, 7182819, 17492756, 18676217, 35914822, 62219662, 74431990, 78514167,
					79676256, 80039680, 83129760, 85589566, 89924741, 108140369, 110491666,
					117470325, 122520629, 158934401, 164568050, 173187020, 173702733, 204378159,
					211337893, 214272137, 215129590, 259810369, 262517995, 278304752, 296858429,
					301041326, 325535563, 345848071, 346492047, 356615386, 358851982, 361269356,
					379285715, 438665602, 459559383, 492127418, 512640740, 514137019,
				],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
		excess: Commitment::from_vec(
			util::from_hex(
				"080af906b3aed1ff7501b2255cb52e4f7be6843b5d93c17f10264525fe93516ba5".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			254, 146, 12, 91, 103, 1, 42, 158, 116, 6, 210, 165, 55, 224, 187, 138, 106, 228, 38,
			45, 12, 119, 148, 157, 212, 120, 68, 10, 107, 211, 134, 131, 168, 39, 218, 236, 186,
			169, 27, 216, 136, 253, 123, 3, 138, 18, 81, 148, 175, 145, 136, 255, 46, 64, 57, 16,
			70, 222, 206, 31, 148, 101, 249, 227,
		])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				"09b2e4dda2d22fc98c31d5f242fc201529ec8b87e1f8beab8dc6b77708d5116065".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [
				134, 61, 29, 6, 171, 6, 25, 220, 232, 127, 47, 160, 206, 78, 175, 152, 214, 110,
				224, 146, 22, 69, 4, 59, 39, 79, 70, 96, 50, 130, 237, 158, 208, 223, 34, 119, 159,
				113, 120, 22, 167, 56, 83, 183, 99, 88, 179, 94, 255, 218, 250, 81, 144, 202, 42,
				224, 55, 146, 151, 121, 116, 217, 79, 114, 1, 29, 34, 50, 107, 93, 227, 202, 59,
				137, 172, 197, 215, 159, 31, 238, 3, 27, 23, 107, 196, 174, 57, 185, 169, 162, 91,
				109, 150, 236, 52, 105, 0, 96, 187, 80, 239, 81, 218, 106, 251, 143, 215, 164, 107,
				134, 33, 156, 212, 182, 16, 6, 150, 207, 151, 223, 190, 125, 46, 101, 206, 193,
				195, 173, 249, 2, 192, 12, 182, 129, 252, 99, 54, 12, 170, 3, 147, 38, 23, 221, 59,
				0, 3, 95, 22, 30, 136, 250, 207, 77, 131, 167, 110, 86, 244, 89, 246, 148, 61, 233,
				102, 161, 114, 99, 92, 176, 156, 133, 154, 110, 17, 105, 55, 165, 243, 219, 104,
				178, 202, 79, 172, 85, 115, 204, 131, 181, 11, 251, 207, 82, 26, 37, 33, 39, 249,
				250, 212, 180, 117, 11, 239, 132, 134, 49, 143, 113, 196, 88, 224, 37, 243, 195,
				169, 14, 190, 185, 248, 134, 204, 28, 224, 109, 176, 222, 186, 62, 206, 243, 109,
				81, 109, 88, 132, 35, 6, 200, 205, 232, 88, 150, 81, 137, 18, 105, 199, 253, 132,
				140, 1, 211, 176, 204, 240, 181, 255, 25, 74, 213, 46, 31, 228, 2, 44, 167, 65,
				233, 227, 75, 51, 16, 145, 182, 255, 104, 120, 241, 217, 200, 124, 203, 8, 20, 163,
				242, 178, 36, 228, 228, 136, 111, 24, 64, 60, 202, 242, 171, 70, 79, 210, 113, 196,
				158, 188, 116, 16, 77, 152, 83, 169, 49, 147, 219, 43, 139, 121, 14, 237, 75, 139,
				39, 2, 110, 206, 11, 9, 31, 208, 90, 150, 231, 115, 66, 236, 155, 36, 230, 113,
				254, 57, 228, 80, 222, 28, 21, 201, 175, 109, 143, 99, 162, 0, 189, 80, 85, 122,
				11, 60, 139, 129, 180, 170, 203, 122, 102, 113, 62, 100, 186, 37, 185, 200, 179,
				150, 66, 84, 224, 74, 170, 177, 83, 130, 25, 33, 215, 108, 211, 52, 84, 117, 185,
				170, 166, 138, 43, 17, 60, 96, 144, 46, 204, 165, 230, 46, 99, 74, 235, 4, 209,
				140, 112, 162, 98, 181, 144, 177, 33, 194, 180, 125, 248, 209, 68, 102, 18, 26, 94,
				76, 93, 69, 33, 24, 152, 74, 122, 142, 84, 154, 203, 254, 76, 139, 35, 43, 78, 131,
				41, 17, 173, 186, 98, 131, 79, 10, 233, 5, 211, 0, 205, 45, 120, 251, 124, 222, 51,
				145, 64, 184, 95, 207, 145, 245, 241, 239, 208, 191, 216, 197, 28, 63, 225, 183,
				110, 149, 115, 201, 249, 235, 42, 227, 110, 50, 226, 99, 58, 67, 166, 225, 51, 171,
				238, 214, 192, 144, 173, 68, 182, 238, 240, 169, 229, 74, 8, 238, 221, 137, 186,
				27, 104, 105, 51, 141, 102, 98, 235, 198, 229, 25, 86, 122, 12, 51, 54, 121, 230,
				127, 110, 112, 209, 57, 246, 184, 29, 94, 103, 203, 42, 86, 223, 251, 97, 91, 203,
				12, 206, 49, 240, 132, 182, 191, 204, 126, 131, 25, 39, 173, 103, 216, 168, 53, 52,
				36, 211, 12, 11, 172, 244, 145, 82, 200, 176, 11, 176, 87, 128, 189, 31, 104, 205,
				86, 0, 108, 96, 240, 135, 68, 151, 10, 69, 115, 94, 16, 5, 145, 123, 138, 30, 29,
				195, 19, 67, 123, 229, 21, 195, 115, 96, 87, 99, 51, 93, 189, 142, 177, 7, 31, 4,
				47, 244, 142, 218, 136, 215, 109, 229, 96, 121, 167, 154, 142, 108, 172, 178, 208,
				96, 116, 12, 25, 122, 66, 199, 198, 224, 206, 167, 27, 95, 165, 105, 191, 132, 75,
				146, 89, 203, 114, 86, 116, 128, 83, 121, 128,
			],
		},
	};
	gen.with_reward(output, kernel)
}

/// Mainnet genesis block
pub fn genesis_main() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(2019, 2, 19).and_hms(15, 22, 36),
		prev_root: Hash::from_hex(
			"000000000000000000020318bbb1fb7840da441e37bd237e211c6b0ce513af51",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"90ffe71cd746676725e78557fd514f9005eb89a3c9a28c37eedab15db33f1fab",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"0ec636bcbcd9e5384b959cbd5e7d8913ab9e40efe0d67a927e5be27ca0918592",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"35c26acec78111de79f61dbfb4147dfd33b675ae3741c379edcabde60ebc0bc3",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
			nonce: 31,
			proof: Proof {
				nonces: vec![
					2625793, 7257334, 13059668, 19850725, 29675543, 78431535, 81837158, 84489040,
					110407756, 110949687, 158150016, 168500580, 169796210, 193652934, 196112371,
					199313033, 254070971, 289707568, 289852683, 297931615, 298673959, 319797718,
					323150978, 332464564, 344366340, 348246754, 350531755, 368150824, 376843542,
					393163228, 411687326, 416188852, 416929220, 422311017, 426851599, 440201156,
					449953044, 461191658, 462556347, 465631845, 480535912, 528986355,
				],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
		excess: Commitment::from_vec(
			util::from_hex(
				"080af906b3aed1ff7501b2255cb52e4f7be6843b5d93c17f10264525fe93516ba5".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			24, 126, 49, 147, 195, 162, 180, 151, 231, 103, 185, 104, 2, 42, 198, 156, 208, 79, 17,
			131, 248, 173, 114, 43, 143, 75, 81, 47, 200, 27, 121, 146, 18, 248, 101, 234, 72, 87,
			78, 35, 105, 99, 255, 231, 93, 152, 29, 17, 118, 142, 179, 89, 255, 242, 150, 100, 73,
			106, 108, 11, 131, 175, 104, 232,
		])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				"09b2e4dda2d22fc98c31d5f242fc201529ec8b87e1f8beab8dc6b77708d5116065".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [
				134, 61, 29, 6, 171, 6, 25, 220, 232, 127, 47, 160, 206, 78, 175, 152, 214, 110,
				224, 146, 22, 69, 4, 59, 39, 79, 70, 96, 50, 130, 237, 158, 208, 223, 34, 119, 159,
				113, 120, 22, 167, 56, 83, 183, 99, 88, 179, 94, 255, 218, 250, 81, 144, 202, 42,
				224, 55, 146, 151, 121, 116, 217, 79, 114, 1, 29, 34, 50, 107, 93, 227, 202, 59,
				137, 172, 197, 215, 159, 31, 238, 3, 27, 23, 107, 196, 174, 57, 185, 169, 162, 91,
				109, 150, 236, 52, 105, 0, 96, 187, 80, 239, 81, 218, 106, 251, 143, 215, 164, 107,
				134, 33, 156, 212, 182, 16, 6, 150, 207, 151, 223, 190, 125, 46, 101, 206, 193,
				195, 173, 249, 2, 192, 12, 182, 129, 252, 99, 54, 12, 170, 3, 147, 38, 23, 221, 59,
				0, 3, 95, 22, 30, 136, 250, 207, 77, 131, 167, 110, 86, 244, 89, 246, 148, 61, 233,
				102, 161, 114, 99, 92, 176, 156, 133, 154, 110, 17, 105, 55, 165, 243, 219, 104,
				178, 202, 79, 172, 85, 115, 204, 131, 181, 11, 251, 207, 82, 26, 37, 33, 39, 249,
				250, 212, 180, 117, 11, 239, 132, 134, 49, 143, 113, 196, 88, 224, 37, 243, 195,
				169, 14, 190, 185, 248, 134, 204, 28, 224, 109, 176, 222, 186, 62, 206, 243, 109,
				81, 109, 88, 132, 35, 6, 200, 205, 232, 88, 150, 81, 137, 18, 105, 199, 253, 132,
				140, 1, 211, 176, 204, 240, 181, 255, 25, 74, 213, 46, 31, 228, 2, 44, 167, 65,
				233, 227, 75, 51, 16, 145, 182, 255, 104, 120, 241, 217, 200, 124, 203, 8, 20, 163,
				242, 178, 36, 228, 228, 136, 111, 24, 64, 60, 202, 242, 171, 70, 79, 210, 113, 196,
				158, 188, 116, 16, 77, 152, 83, 169, 49, 147, 219, 43, 139, 121, 14, 237, 75, 139,
				39, 2, 110, 206, 11, 9, 31, 208, 90, 150, 231, 115, 66, 236, 155, 36, 230, 113,
				254, 57, 228, 80, 222, 28, 21, 201, 175, 109, 143, 99, 162, 0, 189, 80, 85, 122,
				11, 60, 139, 129, 180, 170, 203, 122, 102, 113, 62, 100, 186, 37, 185, 200, 179,
				150, 66, 84, 224, 74, 170, 177, 83, 130, 25, 33, 215, 108, 211, 52, 84, 117, 185,
				170, 166, 138, 43, 17, 60, 96, 144, 46, 204, 165, 230, 46, 99, 74, 235, 4, 209,
				140, 112, 162, 98, 181, 144, 177, 33, 194, 180, 125, 248, 209, 68, 102, 18, 26, 94,
				76, 93, 69, 33, 24, 152, 74, 122, 142, 84, 154, 203, 254, 76, 139, 35, 43, 78, 131,
				41, 17, 173, 186, 98, 131, 79, 10, 233, 5, 211, 0, 205, 45, 120, 251, 124, 222, 51,
				145, 64, 184, 95, 207, 145, 245, 241, 239, 208, 191, 216, 197, 28, 63, 225, 183,
				110, 149, 115, 201, 249, 235, 42, 227, 110, 50, 226, 99, 58, 67, 166, 225, 51, 171,
				238, 214, 192, 144, 173, 68, 182, 238, 240, 169, 229, 74, 8, 238, 221, 137, 186,
				27, 104, 105, 51, 141, 102, 98, 235, 198, 229, 25, 86, 122, 12, 51, 54, 121, 230,
				127, 110, 112, 209, 57, 246, 184, 29, 94, 103, 203, 42, 86, 223, 251, 97, 91, 203,
				12, 206, 49, 240, 132, 182, 191, 204, 126, 131, 25, 39, 173, 103, 216, 168, 53, 52,
				36, 211, 12, 11, 172, 244, 145, 82, 200, 176, 11, 176, 87, 128, 189, 31, 104, 205,
				86, 0, 108, 96, 240, 135, 68, 151, 10, 69, 115, 94, 16, 5, 145, 123, 138, 30, 29,
				195, 19, 67, 123, 229, 21, 195, 115, 96, 87, 99, 51, 93, 189, 142, 177, 7, 31, 4,
				47, 244, 142, 218, 136, 215, 109, 229, 96, 121, 167, 154, 142, 108, 172, 178, 208,
				96, 116, 12, 25, 122, 66, 199, 198, 224, 206, 167, 27, 95, 165, 105, 191, 132, 75,
				146, 89, 203, 114, 86, 116, 128, 83, 121, 128,
			],
		},
	};
	gen.with_reward(output, kernel)
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::core::hash::Hashed;
	use crate::ser;

	#[test]
	fn floonet_genesis_hash() {
		let gen_hash = genesis_floo().hash();
		println!("floonet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_floo()).unwrap();
		println!("floonet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"c811f4a3aa40b6e23927fd25041795cb36426c0400391df78b1e632cca49f66a"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"7e7048c8904e507cf45f0020bfe1f1b83c7d74ca87e85ffb32db4adc29701176"
		);
	}

	#[test]
	fn mainnet_genesis_hash() {
		let gen_hash = genesis_main().hash();
		println!("mainnet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_main()).unwrap();
		println!("mainnet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"4baaf3601253124ea1cc93278dcc5778e1d93a1c22bf4042dc76de66db995ce0"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"7b37c1de7616c43f202335987350957f8a7a00baa3fa4146c7a85e5a9bc01974"
		);
	}
}
