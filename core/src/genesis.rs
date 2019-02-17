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
		timestamp: Utc.ymd(2018, 12, 28).and_hms(20, 48, 4),
		prev_root: Hash::from_hex(
			"00000000000000000017ff4903ef366c8f62e3151ba74e41b8332a126542f538",
		)
		.unwrap(),
		output_root: Hash::from_hex(
			"73b5e0a05ea9e1e4e33b8f1c723bc5c10d17f07042c2af7644f4dbb61f4bc556",
		)
		.unwrap(),
		range_proof_root: Hash::from_hex(
			"667a3ba22f237a875f67c9933037c8564097fa57a3e75be507916de28fc0da26",
		)
		.unwrap(),
		kernel_root: Hash::from_hex(
			"cfdddfe2d938d0026f8b1304442655bbdddde175ff45ddf44cb03bcb0071a72d",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(10_u64.pow(5)),
			secondary_scaling: 1856,
			nonce: 23,
			proof: Proof {
				nonces: vec![
					16994232, 22975978, 32664019, 44016212, 50238216, 57272481, 85779161,
					124272202, 125203242, 133907662, 140522149, 145870823, 147481297, 164952795,
					177186722, 183382201, 197418356, 211393794, 239282197, 239323031, 250757611,
					281414565, 305112109, 308151499, 357235186, 374041407, 389924708, 390768911,
					401322239, 401886855, 406986280, 416797005, 418935317, 429007407, 439527429,
					484809502, 486257104, 495589543, 495892390, 525019296, 529899691, 531685572,
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
				"08df2f1d996cee37715d9ac0a0f3b13aae508d1101945acb8044954aee30960be9".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			25, 176, 52, 246, 172, 1, 12, 220, 247, 111, 73, 101, 13, 16, 157, 130, 110, 196, 123,
			217, 246, 137, 45, 110, 106, 186, 0, 151, 255, 193, 233, 178, 103, 26, 210, 215, 200,
			89, 146, 188, 9, 161, 28, 212, 227, 143, 82, 54, 5, 223, 16, 65, 237, 132, 196, 241,
			39, 76, 133, 45, 252, 131, 88, 0,
		])
		.unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
		commit: Commitment::from_vec(
			util::from_hex(
				"08c12007af16d1ee55fffe92cef808c77e318dae70c3bc70cb6361f49d517f1b68".to_string(),
			)
			.unwrap(),
		),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
			proof: [
				159, 156, 202, 179, 128, 169, 14, 227, 176, 79, 118, 180, 62, 164, 2, 234, 123, 30,
				77, 126, 232, 124, 42, 186, 239, 208, 21, 217, 228, 246, 148, 74, 100, 25, 247,
				251, 82, 100, 37, 16, 146, 122, 164, 5, 2, 165, 212, 192, 221, 167, 199, 8, 231,
				149, 158, 216, 194, 200, 62, 15, 53, 200, 188, 207, 0, 79, 211, 88, 194, 211, 54,
				1, 206, 53, 72, 118, 155, 184, 233, 166, 245, 224, 16, 254, 209, 235, 153, 85, 53,
				145, 33, 186, 218, 118, 144, 35, 189, 241, 63, 229, 52, 237, 231, 39, 176, 202, 93,
				247, 85, 131, 16, 193, 247, 180, 33, 138, 255, 102, 190, 213, 129, 174, 182, 167,
				3, 126, 184, 221, 99, 114, 238, 219, 157, 125, 230, 179, 160, 89, 202, 230, 16, 91,
				199, 57, 158, 225, 142, 125, 12, 211, 164, 78, 9, 4, 155, 106, 157, 41, 233, 188,
				237, 205, 184, 53, 0, 190, 24, 215, 42, 44, 184, 120, 58, 196, 198, 190, 114, 50,
				98, 240, 15, 213, 77, 163, 24, 3, 212, 125, 93, 175, 169, 249, 24, 27, 191, 113,
				89, 59, 169, 40, 87, 250, 144, 159, 118, 171, 232, 92, 217, 5, 179, 152, 249, 247,
				71, 239, 26, 180, 82, 177, 226, 132, 185, 3, 33, 162, 120, 98, 87, 109, 57, 100,
				202, 162, 57, 230, 44, 31, 63, 213, 30, 222, 241, 78, 162, 118, 120, 70, 196, 128,
				72, 223, 110, 5, 17, 151, 97, 214, 43, 57, 157, 1, 59, 87, 96, 17, 159, 174, 144,
				217, 159, 87, 36, 113, 41, 155, 186, 252, 162, 46, 22, 80, 133, 3, 113, 248, 11,
				118, 144, 155, 188, 77, 166, 40, 119, 107, 15, 233, 47, 47, 101, 77, 167, 141, 235,
				148, 34, 218, 164, 168, 71, 20, 239, 71, 24, 12, 109, 146, 232, 243, 65, 31, 72,
				186, 131, 190, 43, 227, 157, 41, 49, 126, 136, 51, 41, 50, 213, 37, 186, 223, 87,
				248, 34, 43, 132, 34, 0, 143, 75, 79, 43, 74, 183, 26, 2, 168, 53, 203, 208, 159,
				69, 107, 124, 33, 68, 113, 206, 127, 216, 158, 15, 52, 206, 1, 101, 109, 199, 13,
				131, 122, 29, 131, 133, 125, 219, 70, 69, 144, 133, 68, 233, 67, 203, 132, 160,
				143, 101, 84, 110, 15, 175, 111, 124, 24, 185, 222, 154, 238, 77, 241, 105, 8, 224,
				230, 43, 178, 49, 95, 137, 33, 227, 118, 207, 239, 56, 21, 51, 220, 22, 48, 162,
				22, 118, 229, 215, 248, 112, 198, 126, 180, 27, 161, 237, 56, 2, 220, 129, 126, 11,
				104, 8, 133, 190, 162, 204, 3, 63, 249, 173, 210, 152, 252, 143, 157, 79, 228, 232,
				230, 72, 164, 131, 183, 151, 230, 219, 186, 21, 34, 154, 219, 215, 231, 179, 47,
				217, 44, 115, 203, 157, 35, 195, 113, 235, 194, 102, 96, 205, 24, 221, 213, 147,
				120, 178, 221, 153, 146, 44, 172, 131, 77, 21, 61, 15, 5, 6, 205, 164, 203, 76,
				228, 29, 126, 136, 88, 230, 210, 62, 164, 103, 125, 55, 231, 129, 89, 61, 222, 50,
				71, 71, 75, 230, 70, 80, 85, 193, 136, 183, 222, 146, 46, 235, 0, 222, 118, 32, 70,
				85, 39, 92, 233, 211, 169, 159, 207, 145, 13, 206, 125, 3, 45, 51, 64, 167, 179,
				133, 83, 57, 190, 51, 239, 211, 74, 116, 75, 71, 248, 249, 184, 13, 31, 129, 107,
				104, 179, 76, 194, 186, 4, 13, 122, 167, 254, 126, 153, 50, 8, 1, 200, 203, 213,
				230, 217, 97, 105, 50, 208, 126, 180, 113, 81, 152, 238, 123, 157, 232, 19, 164,
				159, 164, 89, 75, 33, 70, 140, 204, 158, 236, 10, 226, 102, 14, 88, 134, 82, 131,
				36, 195, 127, 158, 81, 252, 223, 165, 11, 52, 105, 245, 245, 228, 235, 168, 175,
				52, 175, 76, 157, 120, 208, 99, 135, 210, 81, 114, 230, 181,
			],
		},
	};
	gen.with_reward(output, kernel)
}

/// Mainnet genesis block
pub fn genesis_main() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
timestamp: Utc.ymd(2019, 2, 17).and_hms(4, 41, 11),
prev_root: Hash::from_hex("0000000000000000001d6cf3d8d1bf2e0b1883f6684bb2dddcb57a5d4c81ec7f").unwrap(),
output_root: Hash::from_hex("9bfa59f5b998c04d64ae46044cfca718aa3a0f30823c58826907839e1ecfdf4f").unwrap(),
range_proof_root: Hash::from_hex("0a9c8321edfbc772cab1452d7f9d1d9306f5731c9205036ac6ef096829d341b4").unwrap(),
kernel_root: Hash::from_hex("1643b0c827b4b419e1aa9a55234dff17a6d6c7957a4b16ff5f9d96b0df6d7bd6").unwrap(),
total_kernel_offset: BlindingFactor::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
nonce: 253,
			proof: Proof {
nonces: vec![6315885, 24332413, 31172962, 71720963, 75951890, 76627246, 93865124, 102079007, 104552290, 114941996, 118381006, 139252089, 164228456, 188912686, 203645527, 233639313, 246587145, 251681395, 254700253, 293455002, 299960363, 323419161, 327204292, 330266448, 348119393, 382003391, 409209432, 413254364, 420449628, 420615187, 421546924, 447152170, 449474155, 457968359, 460429335, 468563182, 469210740, 479995682, 485259629, 485781906, 521484650, 535669750],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
excess: Commitment::from_vec(util::from_hex("098702fb65e8dc9199f8311ca0429e37cb6c2c4bf711f484c8a8c989d1743af0b5".to_string()).unwrap()),
excess_sig: Signature::from_raw_data(&[4, 54, 25, 251, 196, 242, 216, 220, 238, 71, 118, 20, 76, 171, 91, 114, 30, 51, 213, 196, 10, 58, 42, 181, 31, 72, 136, 105, 105, 6, 142, 204, 137, 33, 119, 123, 114, 165, 238, 86, 202, 25, 187, 126, 236, 42, 161, 86, 191, 211, 102, 17, 79, 59, 19, 125, 76, 46, 75, 248, 92, 165, 48, 52]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
commit: Commitment::from_vec(util::from_hex("09b39ca1cec7838e2a7c93d7c56523ee02cf21486ad28c349b63f99001b8e17098".to_string()).unwrap()),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
proof: [106, 211, 155, 152, 17, 164, 246, 37, 48, 254, 15, 55, 110, 133, 239, 145, 29, 150, 119, 183, 246, 190, 140, 61, 60, 205, 43, 150, 245, 115, 98, 74, 52, 111, 38, 233, 141, 93, 156, 168, 114, 168, 70, 2, 208, 22, 2, 157, 139, 169, 224, 65, 6, 186, 164, 159, 148, 43, 154, 236, 45, 11, 58, 248, 14, 218, 174, 243, 234, 165, 208, 109, 167, 151, 225, 223, 120, 80, 198, 174, 27, 31, 168, 110, 233, 24, 25, 21, 22, 226, 236, 253, 87, 170, 97, 147, 209, 227, 35, 163, 52, 80, 239, 88, 79, 158, 116, 56, 128, 27, 112, 184, 243, 103, 81, 63, 164, 76, 254, 71, 75, 244, 104, 22, 244, 133, 113, 209, 85, 104, 109, 187, 202, 166, 6, 54, 34, 236, 196, 224, 91, 182, 218, 151, 68, 174, 212, 25, 170, 146, 73, 135, 55, 93, 104, 167, 122, 218, 143, 128, 214, 238, 191, 237, 64, 73, 188, 49, 91, 78, 58, 6, 28, 165, 206, 242, 37, 149, 140, 59, 9, 163, 69, 93, 208, 231, 94, 238, 32, 146, 101, 42, 184, 181, 187, 131, 123, 236, 132, 33, 144, 223, 42, 70, 202, 198, 111, 88, 199, 27, 27, 36, 249, 81, 11, 19, 79, 47, 90, 232, 86, 118, 68, 0, 140, 198, 85, 205, 195, 180, 60, 3, 129, 41, 91, 19, 47, 140, 24, 114, 131, 48, 92, 221, 74, 143, 52, 75, 96, 168, 224, 212, 18, 127, 31, 156, 157, 187, 125, 240, 106, 199, 192, 2, 132, 195, 238, 157, 226, 219, 153, 89, 147, 29, 204, 58, 221, 219, 83, 182, 209, 121, 142, 187, 172, 60, 154, 49, 202, 87, 188, 22, 146, 112, 163, 181, 56, 17, 36, 36, 143, 153, 85, 162, 183, 25, 210, 254, 208, 218, 177, 90, 25, 247, 106, 7, 73, 130, 156, 214, 202, 231, 254, 157, 64, 198, 51, 141, 41, 204, 130, 143, 14, 227, 199, 118, 65, 163, 20, 111, 43, 38, 87, 81, 124, 161, 87, 242, 92, 166, 148, 163, 102, 42, 1, 57, 42, 26, 53, 67, 173, 39, 255, 213, 238, 208, 31, 122, 3, 189, 16, 107, 166, 62, 219, 121, 173, 79, 242, 124, 224, 165, 75, 115, 17, 206, 19, 30, 234, 10, 88, 67, 175, 206, 176, 125, 250, 155, 46, 82, 143, 138, 64, 38, 182, 112, 215, 127, 104, 6, 41, 184, 1, 42, 6, 168, 233, 63, 66, 209, 141, 92, 224, 228, 229, 77, 70, 130, 128, 12, 227, 147, 86, 199, 215, 213, 132, 68, 209, 51, 128, 131, 209, 197, 4, 45, 106, 161, 163, 205, 249, 122, 73, 84, 183, 38, 28, 109, 25, 3, 44, 50, 159, 40, 172, 188, 43, 44, 15, 4, 42, 231, 243, 1, 45, 50, 113, 4, 106, 228, 137, 232, 10, 131, 60, 254, 125, 39, 179, 37, 153, 3, 80, 135, 35, 219, 126, 71, 18, 90, 127, 141, 194, 115, 214, 193, 133, 146, 62, 234, 84, 245, 117, 64, 87, 175, 17, 54, 128, 181, 116, 39, 25, 12, 150, 44, 16, 76, 159, 178, 56, 218, 241, 246, 8, 101, 192, 242, 80, 62, 131, 186, 101, 174, 246, 164, 137, 237, 57, 137, 72, 74, 121, 40, 55, 151, 233, 90, 152, 82, 120, 190, 137, 113, 164, 25, 140, 168, 29, 149, 242, 116, 5, 67, 121, 131, 241, 222, 64, 45, 6, 245, 42, 144, 196, 29, 167, 120, 197, 109, 83, 217, 7, 108, 179, 113, 215, 84, 253, 152, 175, 118, 24, 224, 109, 142, 171, 210, 31, 119, 147, 26, 173, 247, 156, 82, 46, 31, 5, 82, 63, 108, 169, 76, 178, 155, 179, 208, 72, 211, 228, 245, 5, 9, 160, 60, 4, 165, 27, 86, 153, 126, 156, 118, 79, 18, 111, 120, 119, 53, 67, 165, 150, 149, 105, 179, 242, 24, 182, 45, 66, 200, 12, 167, 88, 146, 233, 54, 214, 181, 68, 191, 122, 116, 154],
		}
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
			"edc758c1370d43e1d733f70f58cf187c3be8242830429b1676b89fd91ccf2dab"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"91c638fc019a54e6652bd6bb3d9c5e0c17e889cef34a5c28528e7eb61a884dc4"
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
			"c8eec0dff9e756d83afe613fe80eceb3d91fb786257138559596e45caa567864"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"4c4b83f9f8d1bc868a4af6dd11e73f1c6a9a6a939a0109a4073a0af20c0f7e8a"
		);
	}
}
