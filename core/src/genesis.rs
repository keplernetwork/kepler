// Copyright 2019 The Kepler Developers
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
		timestamp: Utc.ymd(1997, 8, 4).and_hms(0, 0, 0),
		pow: ProofOfWork {
			nonce: 0,
			..Default::default()
		},
		..Default::default()
	})
}

/// Floonet genesis block
pub fn genesis_floo() -> core::Block {
	let gen = core::Block::with_header(core::BlockHeader {
		height: 0,
		timestamp: Utc.ymd(2019, 2, 20).and_hms(23, 52, 17),
		prev_root: Hash::from_hex(
			"0000000000000000002271b3d11c1b74d1e4ece2460a1cbbfe1ccbab3c33f881",
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
			"61be20e46142482cd2d9ca3a3c2f7ce5c3e13ab0a4e44d293629fecdd10e8710",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(20)),
			secondary_scaling: 1856,
			nonce: 9,
			proof: Proof {
				nonces: vec![
					6990708, 29134117, 45356420, 72108087, 78984730, 91721010, 105521394,
					113034046, 161595394, 185160252, 190958536, 199335412, 216022394, 237784387,
					245954080, 273869105, 289802646, 290765723, 292324239, 298352804, 313739495,
					322028697, 325878893, 334359638, 342354783, 349112909, 350921429, 371037986,
					372927248, 374724599, 386618656, 389921505, 414419042, 427230377, 434126585,
					441644276, 455803256, 485424841, 488573201, 496066815, 522998324, 530604207,
				],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(
			util::from_hex(
				"080af906b3aed1ff7501b2255cb52e4f7be6843b5d93c17f10264525fe93516ba5".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			254, 246, 236, 95, 176, 78, 141, 128, 246, 216, 250, 238, 223, 183, 185, 105, 90, 57,
			33, 188, 35, 54, 216, 68, 158, 166, 188, 152, 91, 208, 143, 17, 47, 211, 145, 31, 170,
			246, 239, 87, 203, 97, 99, 215, 190, 62, 143, 179, 247, 244, 127, 30, 81, 202, 149, 5,
			180, 211, 56, 245, 184, 90, 217, 163,
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
		timestamp: Utc.ymd(2019, 2, 20).and_hms(23, 55, 17),
		prev_root: Hash::from_hex(
			"000000000000000000210b89a5f133fc870637e30fb69984cf09f6917ff78cd6",
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
			"88e3a59bc96a270733c2e593103d696df58933b29336d5158c7f53e11a3f3f8d",
		)
		.unwrap(),
		total_kernel_offset: BlindingFactor::from_hex(
			"0000000000000000000000000000000000000000000000000000000000000000",
		)
		.unwrap(),
		output_mmr_size: 1,
		kernel_mmr_size: 1,
		pow: ProofOfWork {
			total_difficulty: Difficulty::from_num(2_u64.pow(20)),
			secondary_scaling: 1856,
			nonce: 46,
			proof: Proof {
				nonces: vec![
					21173293, 39668212, 41583454, 42772682, 49987386, 54534419, 67130559, 71059408,
					80664177, 86496331, 136593297, 138808525, 167374354, 180463122, 184898128,
					205639151, 214087070, 220786027, 226664471, 237275233, 246819995, 258314920,
					289589791, 294899049, 309635196, 333385264, 347357955, 347431272, 359321423,
					375967375, 376164192, 377616507, 379425382, 417656757, 419841884, 431057716,
					447422142, 452781500, 463190707, 477834848, 510815761, 516707518,
				],
				edge_bits: 29,
			},
		},
		..Default::default()
	});
	let kernel = core::TxKernel {
		features: core::KernelFeatures::Coinbase,
		excess: Commitment::from_vec(
			util::from_hex(
				"080af906b3aed1ff7501b2255cb52e4f7be6843b5d93c17f10264525fe93516ba5".to_string(),
			)
			.unwrap(),
		),
		excess_sig: Signature::from_raw_data(&[
			242, 198, 183, 81, 255, 29, 21, 208, 52, 182, 231, 196, 44, 54, 206, 181, 108, 98, 19,
			178, 91, 95, 101, 90, 143, 12, 9, 181, 90, 181, 55, 54, 116, 190, 73, 162, 164, 196,
			204, 213, 220, 131, 21, 30, 161, 68, 56, 254, 172, 77, 158, 233, 32, 59, 55, 185, 177,
			246, 77, 65, 242, 194, 110, 29,
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
	use crate::ser::{self, ProtocolVersion};

	#[test]
	fn floonet_genesis_hash() {
		let gen_hash = genesis_floo().hash();
		println!("floonet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_floo(), ProtocolVersion(1)).unwrap();
		println!("floonet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"316366231cb07136c543614d1b29574ba7acfea24125aee02ee651e6cb4381fe"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"c0fd577221d9c4421aea501ab011c94a0df1598eb4fee9fb3f402d4e5f9690d3"
		);
	}

	#[test]
	fn mainnet_genesis_hash() {
		let gen_hash = genesis_main().hash();
		println!("mainnet genesis hash: {}", gen_hash.to_hex());
		let gen_bin = ser::ser_vec(&genesis_main(), ProtocolVersion(1)).unwrap();
		println!("mainnet genesis full hash: {}\n", gen_bin.hash().to_hex());
		assert_eq!(
			gen_hash.to_hex(),
			"541d415584596e192e07e4732c91b9c81d8698c9d3ca4fb9fa1389b93d01d2a6"
		);
		assert_eq!(
			gen_bin.hash().to_hex(),
			"d6b31a6abc2700977ec3a6554aaee0d5a007f0785655cbbb838b1a9d0c102210"
		);
	}
}
