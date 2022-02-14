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

use subtensor_primitives::AccountId;
use sc_chain_spec::{ChainType, Properties};
use serde_json::map::Map;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::sr25519;
use sp_runtime::traits::Zero;

use crate::chain_spec::{get_account_id_from_seed, get_parachain_authority_keys_from_seed, Extensions};

use nakamoto_runtime::{
	Balance, BalancesConfig, BlockNumber, ParachainInfoConfig, PolkadotXcmConfig, SS58Prefix, SessionConfig,
	SessionKeys, SystemConfig,
};

pub type ChainSpec = sc_service::GenericChainSpec<nakamoto_runtime::GenesisConfig, Extensions>;

pub const PARA_ID: u32 = 2000;

pub fn karura_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../../../../resources/karura-dist.json")[..])
}

fn karura_properties() -> Properties {
	let mut properties = Map::new();
	let mut token_symbol: Vec<String> = vec![];
	let mut token_decimals: Vec<u32> = vec![];
	properties.insert("tokenSymbol".into(), token_symbol.into());
	properties.insert("tokenDecimals".into(), token_decimals.into());
	properties.insert("ss58Format".into(), SS58Prefix::get().into());

	properties
}

pub fn karura_dev_config() -> Result<ChainSpec, String> {
	let wasm_binary = nakamoto_runtime::WASM_BINARY.unwrap_or_default();

	Ok(ChainSpec::from_genesis(
		"Acala Karura Dev",
		"karura-dev",
		ChainType::Development,
		move || {
			karura_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![get_parachain_authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![],
				vec![],
				vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
			)
		},
		vec![],
		None,
		None,
		None,
		Some(karura_properties()),
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
			bad_blocks: None,
		},
	))
}

fn karura_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	initial_allocation: Vec<(AccountId, Balance)>,
	vesting_list: Vec<(AccountId, BlockNumber, BlockNumber, u32, Balance)>,
	general_councils: Vec<AccountId>,
) -> nakamoto_runtime::GenesisConfig {
	nakamoto_runtime::GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			balances: initial_allocation,
		},
		parachain_info: ParachainInfoConfig {
			parachain_id: PARA_ID.into(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.cloned()
				.map(|(acc, aura)| {
					(
						acc.clone(),          // account id
						acc,                  // validator id
						SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		collator_selection: CollatorSelectionConfig {
			invulnerables: initial_authorities.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: Zero::zero(),
			..Default::default()
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: PolkadotXcmConfig {
			safe_xcm_version: Some(2),
		},
	}
}
