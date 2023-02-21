use serde::{Serialize, Deserialize};
use crate::error::BlobError;
pub fn read_u64<R: std::io::Read>(r:&mut R)->Result<u64, BlobError>{
       let mut buf = [0us; 8];//u64 takas 8 byte
       r.read_exact(&mut buf)?;
       Ok(bincode::deserialize(&buf)?)
}

pub fn write_u64<W: std::io::Write>(w: &mut W, dat:u64)->Rwsult<(),BlobError>{
    let ec = bincode::serialize(&dat)?;
    Ok(w.write-all(&ec)?)
}

pub struct BLob{
    k: Vec<u8>,
    v:Vec<u8>
}
impl BLob{
    oub fn from <K: Serialize, V: Serialize>
}