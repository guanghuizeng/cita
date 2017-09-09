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

////////////////////////////////////////////////////////////////////////////////


////////////////////////////////////////////////////////////////////////////////

use action_params::ActionParams;
use evm::{self, Ext, GasLeft};
use std::collections::HashMap;
use util::{H256, U256};

////////////////////////////////////////////////////////////////////////////////
pub type Signature = u32;
pub mod storage;
////////////////////////////////////////////////////////////////////////////////
// Contract
pub trait Contract<'a>: Sync + Send {
    fn exec2(&self, signature: Signature, params: ActionParams, ext: &mut Ext) -> Result<GasLeft, evm::Error>;
    fn exec(&'a self, params: &ActionParams, ext: &mut Ext) -> Result<GasLeft<'a>, evm::Error>;
}

////////////////////////////////////////////////////////////////////////////////
// NowPay
pub struct NowPay<'a> {
    funcs: HashMap<Signature, Box<Fn(&'a NowPay, ActionParams, &mut Ext) -> Result<GasLeft<'a>, evm::Error> + Sync + Send>>,
}

impl<'a> Contract for NowPay<'a> {
    fn exec(&self, params: &ActionParams, ext: &mut Ext) {}
    fn exec2(&'a self, signature: Signature, params: ActionParams, ext: &mut Ext) -> Result<GasLeft<'a>, evm::Error> {
        if let Some(func) = self.funcs.get(&signature) {
            func(self, params, ext)
        } else {
            Err(evm::Error::OutOfGas)
        }
    }
}

impl<'a> NowPay<'a> {
    pub fn new() -> Self {
        let mut contract = NowPay { funcs: HashMap::<Signature, Box<Fn(&'a NowPay, ActionParams, &mut Ext) -> evm::Result<GasLeft<'a>> + Sync + Send>>::new() };
        contract.funcs.insert(0, Box::new(NowPay::set_value));
        contract
    }

    pub fn set_value(&'a self, params: ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'a>> {
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..32) {
                let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }
}
