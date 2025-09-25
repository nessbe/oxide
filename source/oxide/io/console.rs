// File:       console.rs
// Project:    oxide
// Repository: https://github.com/nessbe/oxide
//
// Copyright (c) 2025 nessbe
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at:
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// For more details, see the LICENSE file at the root of the project.

use std::io::{self, Write};

pub struct Console;

impl Console {
	pub fn print(message: &str) {
		print!("{}", message);
		io::stdout().flush().unwrap();
	}

	pub fn print_line(message: &str) {
		println!("{}", message);
	}

	pub fn read_line() -> String {
		let mut input = String::new();
		io::stdin().read_line(&mut input).unwrap();
		input.trim().to_string()
	}
}
