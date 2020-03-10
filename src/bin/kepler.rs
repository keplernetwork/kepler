// Copyright 2020 The Kepler Developers
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

//! Main for building the binary of a Kepler peer-to-peer node.

#[macro_use]
extern crate clap;

#[macro_use]
extern crate log;
use crate::config::config::SERVER_CONFIG_FILE_NAME;
use crate::core::global;
use crate::util::init_logger;
use clap::App;
use kepler_api as api;
use kepler_chain as chain;
use kepler_config as config;
use kepler_core as core;
use kepler_p2p as p2p;
use kepler_servers as servers;
use kepler_util as util;
use kepler_util::logger::LogEntry;
use std::sync::mpsc;

mod cmd;
pub mod tui;

// include build information
pub mod built_info {
	include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub fn info_strings() -> (String, String) {
	(
		format!(
			"This is Kepler version {}{}, built for {} by {}.",
			built_info::PKG_VERSION,
			built_info::GIT_VERSION.map_or_else(|| "".to_owned(), |v| format!(" (git {})", v)),
			built_info::TARGET,
			built_info::RUSTC_VERSION,
		)
		.to_string(),
		format!(
			"Built with profile \"{}\", features \"{}\".",
			built_info::PROFILE,
			built_info::FEATURES_STR,
		)
		.to_string(),
	)
}

fn log_build_info() {
	let (basic_info, detailed_info) = info_strings();
	info!("{}", basic_info);
	debug!("{}", detailed_info);
}

fn main() {
	let exit_code = real_main();
	std::process::exit(exit_code);
}

fn real_main() -> i32 {
	let yml = load_yaml!("kepler.yml");
	let args = App::from_yaml(yml)
		.version(built_info::PKG_VERSION)
		.get_matches();
	let node_config;

	// Temporary wallet warning message
	match args.subcommand() {
		("wallet", _) => {
			println!();
			println!("As of v1.1.0, the wallet has been split into a separate executable.");
			println!(
				"Please visit https://github.com/keplernetwork/kepler-wallet/releases to download"
			);
			println!();
			return 0;
		}
		_ => {}
	}

	let chain_type = if args.is_present("floonet") {
		global::ChainTypes::Floonet
	} else if args.is_present("usernet") {
		global::ChainTypes::UserTesting
	} else {
		global::ChainTypes::Mainnet
	};

	// Deal with configuration file creation
	match args.subcommand() {
		("server", Some(server_args)) => {
			// If it's just a server config command, do it and exit
			if let ("config", Some(_)) = server_args.subcommand() {
				cmd::config_command_server(&chain_type, SERVER_CONFIG_FILE_NAME);
				return 0;
			}
		}
		_ => {}
	}

	// Load relevant config
	match args.subcommand() {
		// When the subscommand is 'server' take into account the 'config_file' flag
		("server", Some(server_args)) => {
			if let Some(_path) = server_args.value_of("config_file") {
				node_config = Some(config::GlobalConfig::new(_path).unwrap_or_else(|e| {
					panic!("Error loading server configuration: {}", e);
				}));
			} else {
				node_config = Some(
					config::initial_setup_server(&chain_type).unwrap_or_else(|e| {
						panic!("Error loading server configuration: {}", e);
					}),
				);
			}
		}
		// Otherwise load up the node config as usual
		_ => {
			node_config = Some(
				config::initial_setup_server(&chain_type).unwrap_or_else(|e| {
					panic!("Error loading server configuration: {}", e);
				}),
			);
		}
	}

	let mut config = node_config.clone().unwrap();
	let mut logging_config = config.members.as_mut().unwrap().logging.clone().unwrap();
	logging_config.tui_running = config.members.as_mut().unwrap().server.run_tui;

	let (logs_tx, logs_rx) = if logging_config.tui_running.unwrap() {
		let (logs_tx, logs_rx) = mpsc::sync_channel::<LogEntry>(200);
		(Some(logs_tx), Some(logs_rx))
	} else {
		(None, None)
	};
	init_logger(Some(logging_config), logs_tx);

	global::set_mining_mode(config.members.unwrap().server.clone().chain_type);

	if let Some(file_path) = &config.config_file_path {
		info!(
			"Using configuration file at {}",
			file_path.to_str().unwrap()
		);
	} else {
		info!("Node configuration file not found, using default");
	};

	log_build_info();

	// Execute subcommand
	match args.subcommand() {
		// server commands and options
		("server", Some(server_args)) => {
			cmd::server_command(Some(server_args), node_config.unwrap(), logs_rx)
		}

		// client commands and options
		("client", Some(client_args)) => cmd::client_command(client_args, node_config.unwrap()),

		// clean command
		("clean", _) => {
			let db_root_path = node_config.unwrap().members.unwrap().server.db_root;
			println!("Cleaning chain data directory: {}", db_root_path);
			match std::fs::remove_dir_all(db_root_path) {
				Ok(_) => 0,
				Err(_) => 1,
			}
		}

		// If nothing is specified, try to just use the config file instead
		// this could possibly become the way to configure most things
		// with most command line options being phased out
		_ => cmd::server_command(None, node_config.unwrap(), logs_rx),
	}
}
