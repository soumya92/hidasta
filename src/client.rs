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

use std::io::{Read, Result, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

pub fn wait<P: AsRef<Path>>(path: P) -> Result<()> {
	let mut stream = UnixStream::connect(path)?;
	let mut buf = [0; 1];
	stream.read(&mut buf)?; // returns 0.
	Ok(())
}

pub fn signal<P: AsRef<Path>>(path: P) -> Result<()> {
	let mut stream = UnixStream::connect(path)?;
	stream.write(&[0xffu8])?;
	Ok(())
}
