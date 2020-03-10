// Copyright 2020 The Grin Developers
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

use kepler_core::consensus::{
	secondary_pow_ratio, valid_header_version, FLOONET_FIRST_HARD_FORK, FLOONET_SECOND_HARD_FORK,
	YEAR_HEIGHT,
};
use kepler_core::core::HeaderVersion;
use kepler_core::global;

#[test]
fn test_secondary_pow_ratio() {
	// Tests for Floonet chain type (covers pre and post hardfork).
	global::set_mining_mode(global::ChainTypes::Floonet);
	assert_eq!(global::is_floonet(), true);

	assert_eq!(secondary_pow_ratio(1), 5);
	assert_eq!(secondary_pow_ratio(89), 5);
	assert_eq!(secondary_pow_ratio(90), 5);
	assert_eq!(secondary_pow_ratio(91), 5);
	assert_eq!(secondary_pow_ratio(179), 5);
	assert_eq!(secondary_pow_ratio(180), 5);
	assert_eq!(secondary_pow_ratio(181), 5);

	let one_week = 60 * 24 * 7;
	assert_eq!(secondary_pow_ratio(one_week - 1), 5);
	assert_eq!(secondary_pow_ratio(one_week), 5);
	assert_eq!(secondary_pow_ratio(one_week + 1), 5);

	let two_weeks = one_week * 2;
	assert_eq!(secondary_pow_ratio(two_weeks - 1), 4);
	assert_eq!(secondary_pow_ratio(two_weeks), 4);
	assert_eq!(secondary_pow_ratio(two_weeks + 1), 4);

	let t4_fork_height = 64_000;
	assert_eq!(secondary_pow_ratio(t4_fork_height - 1), 0);
	assert_eq!(secondary_pow_ratio(t4_fork_height), 0);
	assert_eq!(secondary_pow_ratio(t4_fork_height + 1), 0);

	// let one_year = one_week * 52;
	// assert_eq!(secondary_pow_ratio(one_year), 45);

	// let ninety_one_weeks = one_week * 91;
	// assert_eq!(secondary_pow_ratio(ninety_one_weeks - 1), 12);
	// assert_eq!(secondary_pow_ratio(ninety_one_weeks), 12);
	// assert_eq!(secondary_pow_ratio(ninety_one_weeks + 1), 12);

	// let two_year = one_year * 2;
	// assert_eq!(secondary_pow_ratio(two_year - 1), 1);
	// assert_eq!(secondary_pow_ratio(two_year), 0);
	// assert_eq!(secondary_pow_ratio(two_year + 1), 0);
}

#[test]
fn hard_forks() {
	global::set_mining_mode(global::ChainTypes::Floonet);
	assert_eq!(global::is_floonet(), true);
	assert!(valid_header_version(0, HeaderVersion(1)));
	assert!(valid_header_version(10, HeaderVersion(1)));
	assert!(!valid_header_version(10, HeaderVersion(2)));
	assert!(valid_header_version(
		FLOONET_FIRST_HARD_FORK - 1,
		HeaderVersion(1)
	));
	assert!(valid_header_version(
		FLOONET_FIRST_HARD_FORK,
		HeaderVersion(2)
	));
	assert!(valid_header_version(
		FLOONET_FIRST_HARD_FORK + 1,
		HeaderVersion(2)
	));
	assert!(!valid_header_version(
		FLOONET_FIRST_HARD_FORK,
		HeaderVersion(1)
	));
	assert!(!valid_header_version(YEAR_HEIGHT, HeaderVersion(1)));
	assert!(valid_header_version(
		FLOONET_SECOND_HARD_FORK - 1,
		HeaderVersion(2)
	));
	assert!(valid_header_version(
		FLOONET_SECOND_HARD_FORK,
		HeaderVersion(3)
	));
	assert!(valid_header_version(
		FLOONET_SECOND_HARD_FORK + 1,
		HeaderVersion(3)
	));
	assert!(!valid_header_version(
		FLOONET_SECOND_HARD_FORK,
		HeaderVersion(2)
	));
	assert!(!valid_header_version(
		FLOONET_SECOND_HARD_FORK,
		HeaderVersion(1)
	));

	assert!(!valid_header_version(YEAR_HEIGHT - 1, HeaderVersion(2)));
	assert!(valid_header_version(YEAR_HEIGHT - 1, HeaderVersion(3)));
	assert!(valid_header_version(YEAR_HEIGHT, HeaderVersion(3)));
	assert!(valid_header_version(YEAR_HEIGHT + 1, HeaderVersion(3)));
	// v4 not active yet
	assert!(!valid_header_version(YEAR_HEIGHT * 3 / 2, HeaderVersion(4)));
	assert!(!valid_header_version(YEAR_HEIGHT * 3 / 2, HeaderVersion(3)));
	assert!(!valid_header_version(YEAR_HEIGHT * 3 / 2, HeaderVersion(2)));
	assert!(!valid_header_version(YEAR_HEIGHT * 3 / 2, HeaderVersion(1)));
	assert!(!valid_header_version(YEAR_HEIGHT * 2, HeaderVersion(3)));
	assert!(!valid_header_version(
		YEAR_HEIGHT * 3 / 2 + 1,
		HeaderVersion(3)
	));
}
