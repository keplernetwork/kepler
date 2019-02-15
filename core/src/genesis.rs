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
timestamp: Utc.ymd(2019, 2, 14).and_hms(16, 14, 57),
prev_root: Hash::from_hex("0000000000000000001214493875339c04194ecd29bc226f2a7229783aaa8679").unwrap(),
output_root: Hash::from_hex("631439539ed5de766a084996387fa55f1ae8e5777a0ba6e519a820dd0f4919f4").unwrap(),
range_proof_root: Hash::from_hex("1d97d0fed164af2ea197ee501f4269acad36ca0281d0f156a626bdad9a1ab287").unwrap(),
kernel_root: Hash::from_hex("db2317f3d9b43311c43bf97455a55833b7d9ca51bd87e32ef1d4571dd2d5c577").unwrap(),
total_kernel_offset: BlindingFactor::from_hex("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(34)),
			secondary_scaling: 1856,
nonce: 26,
			proof: Proof {
nonces: vec![14095839, 50524502, 74181116, 86643958, 88152658, 96900437, 98340776, 146022304, 149189810, 177875905, 194671107, 198463349, 200763524, 201518676, 219899708, 227436950, 240202847, 262250026, 264699386, 281251046, 281872472, 282886644, 329529440, 332276736, 343297858, 352550123, 369543002, 370969983, 372721247, 384365724, 392984113, 421843349, 425174300, 428850359, 442826208, 445092385, 461293808, 488869384, 505738414, 506151222, 515918452, 519951287],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		fee: 0,
		lock_height: 0,
excess: Commitment::from_vec(util::from_hex("09ac6513a3c7fefed3c67d36ffeb9631e5c4848763d5e73be0914e5f90f0630de1".to_string()).unwrap()),
excess_sig: Signature::from_raw_data(&[181, 164, 172, 122, 233, 54, 186, 249, 189, 25, 229, 180, 221, 58, 20, 36, 155, 94, 251, 181, 160, 222, 178, 159, 96, 116, 72, 121, 211, 210, 16, 55, 191, 243, 103, 76, 147, 214, 192, 45, 52, 253, 238, 171, 197, 203, 212, 107, 190, 209, 36, 200, 183, 240, 105, 134, 112, 37, 238, 59, 81, 27, 131, 152]).unwrap(),
	};
	let output = core::Output {
		features: core::OutputFeatures::Coinbase,
commit: Commitment::from_vec(util::from_hex("099538912437317e890b51352c34b6a73cd218c19ecd666636c2ea431f01b1b84c".to_string()).unwrap()),
		proof: RangeProof {
			plen: SINGLE_BULLET_PROOF_SIZE,
proof: [150, 190, 59, 145, 190, 50, 32, 236, 53, 242, 154, 4, 241, 17, 51, 3, 162, 138, 251, 151, 94, 213, 116, 142, 66, 24, 82, 105, 10, 123, 101, 78, 205, 170, 42, 77, 246, 144, 165, 159, 45, 168, 91, 34, 95, 238, 185, 136, 115, 172, 49, 127, 252, 250, 201, 49, 5, 90, 15, 97, 38, 17, 129, 16, 8, 88, 250, 66, 45, 37, 156, 241, 228, 221, 165, 112, 79, 13, 136, 175, 96, 22, 122, 107, 234, 74, 87, 47, 18, 58, 34, 205, 207, 88, 207, 250, 217, 123, 251, 145, 22, 250, 128, 8, 187, 32, 33, 134, 243, 33, 225, 219, 215, 179, 46, 169, 186, 100, 206, 228, 204, 183, 82, 118, 85, 126, 207, 106, 129, 76, 225, 215, 84, 141, 53, 174, 131, 243, 21, 56, 210, 79, 71, 31, 2, 226, 49, 108, 197, 23, 7, 119, 58, 250, 1, 253, 112, 76, 15, 44, 241, 209, 27, 144, 154, 158, 246, 170, 80, 189, 147, 136, 156, 5, 123, 87, 160, 169, 51, 180, 153, 2, 45, 75, 196, 145, 138, 237, 14, 187, 105, 245, 8, 13, 133, 248, 162, 207, 77, 198, 5, 69, 0, 138, 70, 209, 217, 126, 89, 78, 118, 230, 36, 174, 167, 17, 170, 45, 68, 248, 112, 182, 229, 85, 40, 176, 28, 37, 230, 240, 138, 156, 20, 37, 97, 248, 251, 220, 14, 234, 87, 249, 183, 6, 232, 231, 86, 160, 138, 33, 95, 201, 206, 81, 72, 109, 20, 101, 29, 122, 241, 214, 69, 222, 24, 165, 183, 134, 188, 25, 241, 150, 249, 66, 24, 246, 44, 70, 75, 166, 57, 97, 110, 16, 192, 161, 242, 231, 194, 16, 73, 125, 16, 135, 254, 168, 169, 165, 95, 30, 246, 10, 197, 242, 0, 243, 6, 156, 96, 215, 255, 16, 237, 234, 237, 85, 230, 232, 47, 114, 121, 246, 10, 65, 116, 118, 94, 83, 208, 252, 19, 45, 12, 156, 228, 5, 170, 98, 21, 37, 87, 214, 141, 193, 87, 2, 97, 41, 11, 167, 225, 117, 80, 213, 3, 16, 202, 160, 186, 141, 135, 201, 112, 220, 35, 221, 26, 214, 81, 236, 224, 113, 104, 194, 84, 249, 19, 99, 229, 81, 99, 101, 230, 19, 173, 190, 116, 206, 224, 253, 77, 255, 90, 182, 111, 248, 70, 139, 179, 231, 187, 242, 93, 74, 233, 47, 14, 155, 153, 231, 57, 176, 200, 228, 24, 164, 6, 53, 6, 49, 175, 62, 220, 212, 182, 158, 150, 243, 62, 155, 46, 191, 76, 71, 147, 154, 107, 76, 215, 226, 78, 148, 145, 175, 213, 53, 136, 242, 190, 213, 150, 170, 54, 57, 15, 186, 216, 2, 108, 20, 103, 128, 130, 250, 211, 29, 199, 211, 48, 185, 46, 161, 23, 44, 78, 187, 208, 205, 203, 159, 140, 230, 131, 188, 138, 52, 232, 221, 193, 46, 188, 81, 236, 77, 123, 62, 218, 76, 211, 31, 199, 102, 74, 161, 211, 121, 5, 178, 213, 28, 133, 173, 152, 147, 71, 18, 235, 242, 16, 11, 244, 201, 71, 40, 170, 83, 195, 4, 186, 236, 95, 54, 69, 227, 117, 87, 253, 7, 132, 227, 74, 158, 153, 176, 85, 6, 79, 22, 135, 32, 128, 192, 140, 78, 249, 123, 11, 176, 174, 35, 234, 148, 62, 164, 56, 119, 2, 211, 140, 70, 241, 139, 191, 171, 52, 253, 123, 138, 235, 231, 13, 235, 160, 62, 45, 85, 36, 118, 123, 244, 157, 106, 251, 73, 209, 48, 253, 233, 241, 11, 235, 180, 195, 134, 103, 132, 210, 212, 72, 128, 51, 109, 137, 202, 65, 187, 215, 163, 217, 223, 193, 71, 171, 202, 200, 56, 217, 161, 30, 128, 63, 101, 88, 30, 82, 89, 99, 22, 71, 186, 162, 194, 232, 237, 38, 75, 49, 195, 73, 80, 75, 148, 85, 16, 81, 243, 189, 68, 38, 178, 136, 1, 98, 87, 126, 108, 250, 253, 242, 40, 212, 139, 225, 43, 207],
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
			"1dceffe4440ffafdfca453ebd4082c65355f455c78153408237f74d03c232536"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"c0116374f612623514ade90fefdb1c074f063df3e26aea65099655738d438b3f"
		);
	}
}
