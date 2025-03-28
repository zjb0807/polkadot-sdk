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

//! Autogenerated weights for `pallet_referenda`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-02-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `e0f303704c84`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/collectives-westend-runtime/collectives_westend_runtime.wasm
// --pallet=pallet_referenda
// --header=/__w/polkadot-sdk/polkadot-sdk/cumulus/file_header.txt
// --output=./cumulus/parachains/runtimes/collectives/collectives-westend/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096
// --no-storage-info
// --no-min-squares
// --no-median-slopes

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_referenda`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_referenda::WeightInfo for WeightInfo<T> {
	/// Storage: `AmbassadorCollective::Members` (r:1 w:0)
	/// Proof: `AmbassadorCollective::Members` (`max_values`: None, `max_size`: Some(42), added: 2517, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:0 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn submit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `254`
		//  Estimated: `159279`
		// Minimum execution time: 26_280_000 picoseconds.
		Weight::from_parts(27_583_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365`
		//  Estimated: `317568`
		// Minimum execution time: 51_856_000 picoseconds.
		Weight::from_parts(53_756_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1164`
		//  Estimated: `159279`
		// Minimum execution time: 110_259_000 picoseconds.
		Weight::from_parts(129_048_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1172`
		//  Estimated: `159279`
		// Minimum execution time: 113_384_000 picoseconds.
		Weight::from_parts(125_747_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `701`
		//  Estimated: `317568`
		// Minimum execution time: 154_089_000 picoseconds.
		Weight::from_parts(237_608_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn place_decision_deposit_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `600`
		//  Estimated: `317568`
		// Minimum execution time: 69_683_000 picoseconds.
		Weight::from_parts(71_743_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_decision_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `317`
		//  Estimated: `4365`
		// Minimum execution time: 29_900_000 picoseconds.
		Weight::from_parts(30_905_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn refund_submission_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `167`
		//  Estimated: `4365`
		// Minimum execution time: 14_298_000 picoseconds.
		Weight::from_parts(14_987_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `317568`
		// Minimum execution time: 35_731_000 picoseconds.
		Weight::from_parts(36_747_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Retries` (r:0 w:1)
	/// Proof: `Scheduler::Retries` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	fn kill() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `411`
		//  Estimated: `317568`
		// Minimum execution time: 62_566_000 picoseconds.
		Weight::from_parts(64_081_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_queue_empty() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `140`
		//  Estimated: `3636`
		// Minimum execution time: 11_698_000 picoseconds.
		Weight::from_parts(12_362_000, 0)
			.saturating_add(Weight::from_parts(0, 3636))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1411`
		//  Estimated: `159279`
		// Minimum execution time: 101_346_000 picoseconds.
		Weight::from_parts(146_910_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn one_fewer_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1525`
		//  Estimated: `159279`
		// Minimum execution time: 102_013_000 picoseconds.
		Weight::from_parts(110_210_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_insertion() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `4365`
		// Minimum execution time: 43_920_000 picoseconds.
		Weight::from_parts(52_557_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_requeued_slide() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `935`
		//  Estimated: `4365`
		// Minimum execution time: 43_333_000 picoseconds.
		Weight::from_parts(51_712_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `951`
		//  Estimated: `4365`
		// Minimum execution time: 51_743_000 picoseconds.
		Weight::from_parts(61_818_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::TrackQueue` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::TrackQueue` (`max_values`: None, `max_size`: Some(171), added: 2646, mode: `MaxEncodedLen`)
	fn nudge_referendum_not_queued() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `959`
		//  Estimated: `4365`
		// Minimum execution time: 51_969_000 picoseconds.
		Weight::from_parts(56_872_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_no_deposit() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `262`
		//  Estimated: `159279`
		// Minimum execution time: 23_590_000 picoseconds.
		Weight::from_parts(24_523_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_preparing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `310`
		//  Estimated: `159279`
		// Minimum execution time: 24_155_000 picoseconds.
		Weight::from_parts(25_093_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	fn nudge_referendum_timed_out() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `208`
		//  Estimated: `4365`
		// Minimum execution time: 15_623_000 picoseconds.
		Weight::from_parts(16_278_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_failing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `545`
		//  Estimated: `159279`
		// Minimum execution time: 40_380_000 picoseconds.
		Weight::from_parts(41_343_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::DecidingCount` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_deciding_passing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `646`
		//  Estimated: `159279`
		// Minimum execution time: 80_025_000 picoseconds.
		Weight::from_parts(88_956_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_begin_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `159279`
		// Minimum execution time: 106_597_000 picoseconds.
		Weight::from_parts(133_288_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_end_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `682`
		//  Estimated: `159279`
		// Minimum execution time: 126_869_000 picoseconds.
		Weight::from_parts(143_083_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_not_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `159279`
		// Minimum execution time: 114_095_000 picoseconds.
		Weight::from_parts(147_290_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_continue_confirming() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `703`
		//  Estimated: `159279`
		// Minimum execution time: 70_631_000 picoseconds.
		Weight::from_parts(81_326_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:2 w:2)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:1 w:1)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn nudge_referendum_approved() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `703`
		//  Estimated: `317568`
		// Minimum execution time: 121_161_000 picoseconds.
		Weight::from_parts(157_050_000, 0)
			.saturating_add(Weight::from_parts(0, 317568))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorCollective::MemberCount` (r:1 w:0)
	/// Proof: `AmbassadorCollective::MemberCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:1 w:1)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(155814), added: 158289, mode: `MaxEncodedLen`)
	fn nudge_referendum_rejected() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `699`
		//  Estimated: `159279`
		// Minimum execution time: 107_304_000 picoseconds.
		Weight::from_parts(155_388_000, 0)
			.saturating_add(Weight::from_parts(0, 159279))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:0 w:1)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn set_some_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `314`
		//  Estimated: `4365`
		// Minimum execution time: 22_134_000 picoseconds.
		Weight::from_parts(23_291_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `AmbassadorReferenda::ReferendumInfoFor` (r:1 w:0)
	/// Proof: `AmbassadorReferenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(900), added: 3375, mode: `MaxEncodedLen`)
	/// Storage: `AmbassadorReferenda::MetadataOf` (r:1 w:1)
	/// Proof: `AmbassadorReferenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn clear_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `285`
		//  Estimated: `4365`
		// Minimum execution time: 18_374_000 picoseconds.
		Weight::from_parts(19_127_000, 0)
			.saturating_add(Weight::from_parts(0, 4365))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
