// This file is part of Acala.

// Copyright (C) 2020-2022 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for module_cdp_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-27, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("acala-latest"), DB CACHE: 128

// Executed Command:
// target/release/acala
// benchmark
// --chain=acala-latest
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./templates/runtime-weight-template.hbs
// --output=./runtime/acala/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for module_cdp_treasury.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> module_cdp_treasury::WeightInfo for WeightInfo<T> {
	// Storage: Tokens Accounts (r:1 w:0)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:1)
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:1 w:0)
	// Storage: AuctionManager TotalTargetInAuction (r:1 w:1)
	// Storage: Auction AuctionsIndex (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AuctionManager CollateralAuctions (r:0 w:1)
	// Storage: Auction AuctionEndTime (r:0 w:1)
	// Storage: Auction Auctions (r:0 w:1)
	fn auction_collateral(b: u32, ) -> Weight {
		(2_672_000 as Weight)
			// Standard Error: 326_000
			.saturating_add((32_334_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
	// Storage: Tokens Accounts (r:4 w:4)
	// Storage: AuctionManager TotalCollateralInAuction (r:1 w:0)
	// Storage: Dex TradingPairStatuses (r:1 w:0)
	// Storage: Dex LiquidityPool (r:1 w:1)
	// Storage: System Account (r:2 w:1)
	fn exchange_collateral_to_stable() -> Weight {
		(176_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(9 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: CdpTreasury ExpectedCollateralAuctionSize (r:0 w:1)
	fn set_expected_collateral_auction_size() -> Weight {
		(25_000_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:2 w:1)
	fn extract_surplus_to_treasury() -> Weight {
		(75_000_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}
