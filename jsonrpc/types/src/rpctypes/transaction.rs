// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use serde_types::hash::H256;
use libproto::request::{FullTransaction as PTransaction};
use libproto::blockchain::{Transaction as ProtoTransaction,};
use bytes::Bytes;
use protobuf::Message;
use serde_types::U256;

// TODO: No need Deserialize. Just because test in trans.rs
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct FullTransaction {
    pub hash: H256,
    pub content: Bytes,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct RpcTransaction {
    pub hash: H256,
    pub content: Bytes,
    #[serde(rename = "blockNumber")]
    pub block_number: U256,
    #[serde(rename = "blockHash")]
    pub block_hash: H256,
    pub index: U256,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TransactionHash {
    pub hash: H256,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum BlockTransaction {
    Full(FullTransaction),
    Hash(TransactionHash),
}

impl From<PTransaction> for RpcTransaction {
    fn from(mut ptransaction: PTransaction) -> Self {
        let transaction = ptransaction.take_transaction();
        let mut bhash: H256 = H256::default();
        bhash.0.clone_from_slice(ptransaction.block_hash.as_slice());

        RpcTransaction {
            hash: transaction.sha3().into(),
            content: Bytes(transaction.write_to_bytes().unwrap()),
            block_number: U256::from(ptransaction.block_number),
            block_hash: bhash,
            index: U256::from(ptransaction.index),
        }
    }
}

impl From<ProtoTransaction> for FullTransaction {
    fn from(transaction: ProtoTransaction) -> Self {
        FullTransaction {
            hash: transaction.sha3().into(),
            content: Bytes(transaction.write_to_bytes().unwrap()),
        }
    }
}

impl From<ProtoTransaction> for TransactionHash {
    fn from(transaction: ProtoTransaction) -> Self {
        TransactionHash { hash: transaction.sha3().into() }
    }
}



