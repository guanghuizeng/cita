use super::*;
use util::*;
use native::types::KV;

#[allow(dead_code, unused_variables)]
pub struct SimpleStorage {
    functions: HashMap<Signature, Box<Function>>,
    uint: U256,
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
        };
        contract.functions.insert(0, Box::new(SimpleStorage::set));
        contract.functions.insert(1, Box::new(SimpleStorage::get));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::*;
    use native::types::KV;

    #[test]
    fn test_simple() {
        let origin = Vec::<u8>::from(concat!("12345678901234567890", "12345678901"));
        println!("origin={:?}", origin);
        let umod = origin.to_hex();
        println!("umod={:?}", umod);
    }


}