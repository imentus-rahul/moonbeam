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

use crate::{
	mock::{
		Account::{Alice, Bob, Charlie, Precompile},
		Call, Event, ExtBuilder, Origin, PrecompilesValue, ProxyType, Runtime, Balances,
	},
	Action,
};
use frame_support::{assert_ok, dispatch::Dispatchable};
use pallet_balances::Call as BalanceCall;
use pallet_proxy::{
	Call as ProxyCall, Event as ProxyEvent, Pallet as ProxyPallet, ProxyDefinition,
};
use precompile_utils::{assert_event_emitted, prelude::*, testing::*};
use sp_core::H160;
use std::str::from_utf8;

#[test]
fn test_selector_less_than_four_bytes_reverts() {
	ExtBuilder::default().build().execute_with(|| {
		PrecompilesValue::get()
			.prepare_test(Alice, Precompile, vec![1u8, 2, 3])
			.execute_reverts(|output| output == b"tried to parse selector out of bounds");
	});
}

#[test]
fn test_unimplemented_selector_reverts() {
	ExtBuilder::default().build().execute_with(|| {
		PrecompilesValue::get()
			.prepare_test(Alice, Precompile, vec![1u8, 2, 3, 4])
			.execute_reverts(|output| output == b"unknown selector");
	});
}

#[test]
fn test_selectors_match_with_actions() {
	assert_eq!(Action::Proxy as u32, 0x93cb5160);
	assert_eq!(Action::ProxyForceType as u32, 0xaec65df0);
	assert_eq!(Action::AddProxy as u32, 0xac69400b);
	assert_eq!(Action::RemoveProxy as u32, 0x78a804c5);
	assert_eq!(Action::RemoveProxies as u32, 0x14a5b5fa);
	assert_eq!(Action::Announce as u32, 0x32cf4272);
	assert_eq!(Action::RemoveAnnouncement as u32, 0x4400aae3);
	assert_eq!(Action::RejectAnnouncement as u32, 0xe508ff89);
	assert_eq!(Action::ProxyAnnounced as u32, 0x8a53f3f5);
	assert_eq!(Action::ProxyForceTypeAnnounced as u32, 0xaf97d7af);
}

#[test]
fn test_add_proxy_fails_if_invalid_value_for_proxy_type() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::AddProxy)
						.write::<Address>(bob.into())
						.write::<u8>(10u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_reverts(|output| output == b"failed decoding proxy_type");
		})
}

#[test]
fn test_add_proxy_fails_if_duplicate_proxy() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::Something,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::AddProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::Something as u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_reverts(|output| from_utf8(&output).unwrap().contains("Duplicate"));
		})
}

#[test]
fn test_add_proxy_succeeds() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::AddProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::Something as u8)
						.write::<u32>(1)
						.build(),
				)
				.execute_returns(vec![]);
			assert_event_emitted!(Event::Proxy(ProxyEvent::ProxyAdded {
				delegator: Alice,
				delegatee: Bob,
				proxy_type: ProxyType::Something,
				delay: 1,
			}));

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(
				proxies,
				vec![ProxyDefinition {
					delegate: Bob,
					proxy_type: ProxyType::Something,
					delay: 1,
				}],
			)
		})
}

#[test]
fn test_add_proxy_multiple_call_adds_less_permissive_type() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::All,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::AddProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::Something as u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_returns(vec![]);
			assert_event_emitted!(Event::Proxy(ProxyEvent::ProxyAdded {
				delegator: Alice,
				delegatee: Bob,
				proxy_type: ProxyType::Something,
				delay: 0,
			}));

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(
				proxies,
				vec![
					ProxyDefinition {
						delegate: Bob,
						proxy_type: ProxyType::All,
						delay: 0,
					},
					ProxyDefinition {
						delegate: Bob,
						proxy_type: ProxyType::Something,
						delay: 0,
					}
				],
			)
		})
}

#[test]
fn test_add_proxy_multiple_call_adds_more_permissive_type() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::Something,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::AddProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::All as u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_returns(vec![]);
			assert_event_emitted!(Event::Proxy(ProxyEvent::ProxyAdded {
				delegator: Alice,
				delegatee: Bob,
				proxy_type: ProxyType::All,
				delay: 0,
			}));

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(
				proxies,
				vec![
					ProxyDefinition {
						delegate: Bob,
						proxy_type: ProxyType::All,
						delay: 0,
					},
					ProxyDefinition {
						delegate: Bob,
						proxy_type: ProxyType::Something,
						delay: 0,
					}
				],
			)
		})
}

#[test]
fn test_remove_proxy_fails_if_invalid_value_for_proxy_type() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::Something,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::RemoveProxy)
						.write::<Address>(bob.into())
						.write::<u8>(10u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_reverts(|output| output == b"failed decoding proxy_type");
		})
}

#[test]
fn test_remove_proxy_fails_if_proxy_not_exist() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::RemoveProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::Something as u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_reverts(|output| from_utf8(&output).unwrap().contains("NotFound"));
		})
}

#[test]
fn test_remove_proxy_succeeds() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::Something,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let bob: H160 = Bob.into();
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::RemoveProxy)
						.write::<Address>(bob.into())
						.write::<u8>(ProxyType::Something as u8)
						.write::<u32>(0)
						.build(),
				)
				.execute_returns(vec![]);
			assert_event_emitted!(Event::Proxy(ProxyEvent::ProxyRemoved {
				delegator: Alice,
				delegatee: Bob,
				proxy_type: ProxyType::Something,
				delay: 0,
			}));

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(proxies, vec![])
		})
}

#[test]
fn test_remove_proxies_succeeds() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::Something,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Charlie,
				proxy_type: ProxyType::All,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::RemoveProxies).build(),
				)
				.execute_returns(vec![]);

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(proxies, vec![])
		})
}

#[test]
fn test_remove_proxies_succeeds_when_no_proxy_exists() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			PrecompilesValue::get()
				.prepare_test(
					Alice,
					Precompile,
					EvmDataWriter::new_with_selector(Action::RemoveProxies).build(),
				)
				.execute_returns(vec![]);

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(proxies, vec![])
		})
}

use parity_scale_codec::Encode;

#[test]
fn test_proxy() {
	ExtBuilder::default()
		.with_balances(vec![(Alice, 1000), (Bob, 1000)])
		.build()
		.execute_with(|| {
			assert_ok!(Call::Proxy(ProxyCall::add_proxy {
				delegate: Bob,
				proxy_type: ProxyType::All,
				delay: 0u64,
			})
			.dispatch(Origin::signed(Alice)));

			let call = Call::Balances(BalanceCall::transfer {
				dest: Charlie,
				value: 1000u128,
			});
			let mut call_bytes:Vec<u8> =
				call.encode();
			let mut decoded_call: xcm::DoubleEncoded<<Runtime as frame_system::Config>::Call> =
			call_bytes.into();

			let bob: H160 = Bob.into();
			let alice: H160 = Alice.into();
			let wc: Vec<u8> = decoded_call.encode();
			PrecompilesValue::get()
				.prepare_test(
					Bob,
					Precompile,
					EvmDataWriter::new_with_selector(Action::Proxy)
						.write::<Address>(alice.into())
						.write(wc)
						.build(),
				)
				.execute_returns2(vec![]);
			println!("{:?}", crate::mock::events());
			assert_event_emitted!(Event::Proxy(ProxyEvent::ProxyAdded {
				delegator: Alice,
				delegatee: Bob,
				proxy_type: ProxyType::Something,
				delay: 1,
			}));

			let proxies = <ProxyPallet<Runtime>>::proxies(Alice).0;
			assert_eq!(
				proxies,
				vec![ProxyDefinition {
					delegate: Bob,
					proxy_type: ProxyType::Something,
					delay: 1,
				}],
			)
		})
}
