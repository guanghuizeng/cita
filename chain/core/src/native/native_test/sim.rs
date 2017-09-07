use super::*;
use util::*;
use native::types::KV;

#[allow(dead_code, unused_variables)]
pub struct SimpleStorage {
    functions: HashMap<Signature, Box<Function>>,
    uint: U256,
    bytes: Vec<u8>,
    string: String,
    //array_u256: Array<U256>,
}

#[allow(dead_code)]
impl Contract for SimpleStorage {
    fn get_function(&self, hash: &Signature) -> Option<&Box<Function>> {
        //info!("function hash={:?}", self.functions.get(hash));
        self.functions.get(hash)
    }
}

#[allow(dead_code, unused_variables)]
impl SimpleStorage {
    pub fn new() -> Self {
        let mut contract = SimpleStorage {
            functions: HashMap::<Signature, Box<Function>>::new(),
            uint: U256::from(0),
            bytes: Vec::new(),
            string: String::new(),
        };
        contract.functions.insert(0, Box::new(SimpleStorage::set));
        contract.functions.insert(1, Box::new(SimpleStorage::get));
        contract.functions.insert(2, Box::new(SimpleStorage::set_bytes));
        contract.functions.insert(3, Box::new(SimpleStorage::get_bytes));
        contract.functions.insert(4, Box::new(SimpleStorage::set_string));
        contract.functions.insert(5, Box::new(SimpleStorage::get_string));
        contract
    }

    pub fn set(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        /*
        if let Some(ref data) = params.data {
            if let Some(data) = data.get(4..36) {
                let _ = ext.set_storage(H256::from(0), H256::from(data));
            }
        }
        */

        if let Some(ref data) = params.data{
            if let Some(data) = data.get(4..36){
                info!("data={:?}, U256::from(data)={:?}", data, U256::from(data));
                let _ = U256::put(ext, &H256::from(0), &U256::from(data));
            }
        }

        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
         if let Ok(str) = U256::get(ext, &H256::from(0)){
             info!("get_value={:?}", str);
         }else {
             trace!("exception for func of get_value");
         }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_bytes(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data{
            let n = data.len();
            if let Some(data) = data.get(4..n){
                info!("n={:?}, data={:?}", n, data);
                let _ = Vec::<u8>::put(ext, &H256::from(1), &data.to_vec());
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_bytes(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(bytes) = Vec::<u8>::get(ext, &H256::from(1)){
            info!("get_bytes={:?}", bytes);
        }else {
            trace!("exception for func of get_bytes");
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn set_string(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Some(ref data) = params.data{
            let n = data.len();
            if let Some(data) = data.get(4..n){
                info!("n={:?}, data={:?}", n, data);
                let _ = String::put(ext, &H256::from(1), &String::from_utf8(data.to_vec()).unwrap());
                //let _ = String::put(ext, &H256::from(2), &data.to_vec().to_hex());
            }
        }
        Ok(GasLeft::Known(U256::from(0)))
    }

    pub fn get_string(params: &ActionParams, ext: &mut Ext) -> evm::Result<GasLeft<'static>> {
        if let Ok(str) = String::get(ext, &H256::from(2)){
            info!("get_string={:?}", *str.as_ref());
        }else {
            trace!("exception for func of get_string");
        }
        Ok(GasLeft::Known(U256::from(0)))
    }
}
