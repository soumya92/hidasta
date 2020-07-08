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

use mio::net::{UnixListener, UnixStream};
use mio::{Events, Interest, Poll, Token};
use slab::Slab;
use std::io::{Read, Result};
use std::path::Path;

const LISTENER: Token = Token(0);

pub fn run<P: AsRef<Path> + Clone>(path: P) -> Result<()> {
	let socket = UnixListener::bind(path.clone())?;
	daemonize().and_then(|is_child| {
		if is_child {
			listen(socket, path)
		} else {
			Ok(())
		}
	})
}

fn listen<P: AsRef<Path>>(mut socket: UnixListener, path: P) -> Result<()> {
	let mut poll = Poll::new()?;
	let mut buf = [0; 1];

	let mut events = Events::with_capacity(16);
	let mut conns: Slab<UnixStream> = Slab::with_capacity(64);

	poll.registry()
		.register(&mut socket, LISTENER, Interest::READABLE)?;

	loop {
		poll.poll(&mut events, None)?;
		for event in events.iter() {
			match event.token() {
				LISTENER => {
					let (conn, _) = socket.accept()?;
					let entry = conns.vacant_entry();
					let token = socket_token(entry.key());
					let conn = entry.insert(conn);
					poll.registry().register(conn, token, Interest::READABLE)?;
				}
				token => {
					let idx = socket_idx(token);
					if conns[idx]
						.read(&mut buf)
						.map(|len| len > 0)
						.unwrap_or_default()
					{
						std::fs::remove_file(path)?;
						for conn in conns.drain() {
							conn.shutdown(std::net::Shutdown::Both)?;
						}
						return Ok(());
					} else {
						conns.remove(idx);
					}
				}
			}
		}
	}
}

fn daemonize() -> Result<bool> {
	use nix::unistd::{close, fork, setsid, ForkResult};
	use std::io;
	use std::process::exit;

	fork()
		.and_then(|r| match r {
			ForkResult::Child => {
				setsid()?;
				close(0)?;
				close(1)?;
				close(2)?;
				fork().map(|r| match r {
					ForkResult::Parent { .. } => exit(0),
					ForkResult::Child => true,
				})
			}
			ForkResult::Parent { .. } => Ok(false),
		})
		.map_err(|e| {
			if let Some(errno) = e.as_errno() {
				io::Error::from(errno)
			} else {
				io::Error::new(io::ErrorKind::Other, e)
			}
		})
}

fn socket_token(idx: usize) -> Token {
	Token(idx + 1)
}

fn socket_idx(token: Token) -> usize {
	token.0 - 1
}
