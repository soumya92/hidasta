// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use getopts::Options;
use std::env;
use std::error::Error;
use std::io;

fn print_usage(program: &str, opts: Options) {
	let brief = format!("Usage: {} SOCKETPATH [options]", program);
	print!("{}", opts.usage(&brief));
}

fn main() -> Result<(), Box<dyn Error>> {
	let args: Vec<String> = env::args().collect();
	let program = args[0].clone();

	let mut opts = Options::new();
	opts.optflag("s", "signal", "signal waiting processes");
	opts.optflag("w", "wait", "wait for a signal [default]");
	opts.optflag("h", "help", "print this help menu");

	let matches = opts.parse(&args[1..])?;
	if matches.opt_present("h") {
		return Ok(print_usage(&program, opts));
	}

	let signal = matches.opt_present("s");
	if signal && matches.opt_present("w") {
		return Ok(print_usage(&program, opts));
	};

	let socket_path = if !matches.free.is_empty() {
		matches.free[0].clone()
	} else {
		return Ok(print_usage(&program, opts));
	};

	let result = if signal {
		hidasta::client::signal(&socket_path)
	} else {
		hidasta::client::wait(&socket_path)
	};

	match result {
		Err(e) if missing_socket(&e) => {
			if signal {
				// no socket == no processes to signal.
				return Ok(());
			}
			hidasta::server::run(&socket_path)?;
			hidasta::client::wait(&socket_path)
		}
		r => r,
	}
	.map_err(Box::from)
}

fn missing_socket(e: &io::Error) -> bool {
	match e.kind() {
		io::ErrorKind::NotFound | io::ErrorKind::ConnectionRefused => true,
		_ => false,
	}
}
