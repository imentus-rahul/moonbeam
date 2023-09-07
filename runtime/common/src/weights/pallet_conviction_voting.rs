// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `pallet_conviction_voting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-08-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ip-10-0-0-176`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("moonbase-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// pallet
// --chain=moonbase-dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --extrinsic=*
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/common/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_conviction_voting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1862`
		//  Estimated: `42428`
		// Minimum execution time: 49_957_000 picoseconds.
		Weight::from_parts(52_161_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2163`
		//  Estimated: `83866`
		// Minimum execution time: 67_364_000 picoseconds.
		Weight::from_parts(68_795_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1807`
		//  Estimated: `83866`
		// Minimum execution time: 46_905_000 picoseconds.
		Weight::from_parts(48_967_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1351`
		//  Estimated: `4617`
		// Minimum execution time: 17_599_000 picoseconds.
		Weight::from_parts(18_229_000, 0)
			.saturating_add(Weight::from_parts(0, 4617))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:20 w:20)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 20]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1505 + r * (235 ±0)`
		//  Estimated: `83866 + r * (3387 ±0)`
		// Minimum execution time: 31_672_000 picoseconds.
		Weight::from_parts(28_035_249, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 84_737
			.saturating_add(Weight::from_parts(18_371_177, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3387).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:20 w:20)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(912), added: 3387, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 20]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1290 + r * (235 ±0)`
		//  Estimated: `83866 + r * (3387 ±0)`
		// Minimum execution time: 17_393_000 picoseconds.
		Weight::from_parts(7_762_064, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 93_875
			.saturating_add(Weight::from_parts(18_253_845, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3387).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(1152), added: 3627, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(119), added: 2594, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1287), added: 3762, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(37), added: 2512, mode: MaxEncodedLen)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1111`
		//  Estimated: `4752`
		// Minimum execution time: 38_078_000 picoseconds.
		Weight::from_parts(38_922_000, 0)
			.saturating_add(Weight::from_parts(0, 4752))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}