// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_std]
#![no_main]
include!("../panic_handler.rs");

use uapi::{input, HostFn, HostFnImpl as api, StorageFlags};

static BUFFER: [u8; 512] = [0u8; 512];

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn deploy() {}

#[no_mangle]
#[polkavm_derive::polkavm_export]
pub extern "C" fn call() {
	input!(len: u32, );

	let rounds = len as usize / BUFFER.len();
	let rest = len as usize / BUFFER.len();
	for i in 0..rounds {
		api::set_storage(StorageFlags::TRANSIENT, &i.to_le_bytes(), &BUFFER);
	}
	api::set_storage(StorageFlags::TRANSIENT, &u32::MAX.to_le_bytes(), &BUFFER[..rest]);
}
