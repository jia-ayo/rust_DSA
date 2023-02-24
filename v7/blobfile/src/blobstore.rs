use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use crate::error::BlobError;
use crate::blob::{Blob,write_u64,read_u64};

const CONT_SIZE: u64 = 32;

///this blob store will act as one half of the hashmap
/// as with the hashmap wrap this  in something that makes growing work 
pub struct BlobStore{
    file:File,
    hseed:u64,
    block_size: u64,
    nblocks: u64,
    elems: u64,
}

impl BlobStore {
    pub fn new (fname:&str, block_size:u64, nblocks:u64)->Result<Self, BlobError>{
        let hseed = rand::random::<u64>();
        //create_file
        let mut ff = OpenOptions::new()
            .create_new(true)
            .write(true)
            .read(true)
            .open(fname)?;
       let f = &mut ff;
       f.set_len(CONT_SIZE + block_size * nblocks)?;
       f.seek(SeekFrom::Start(0))?;
       write_u64(f,hseed)?;
       write_u64(f,block_size)?;
       write_u64(f,nblocks)?;
       write_u64(f,0)?;//0 elems in new store

       //mark beginnings of eack block to show empty
       for x in 0..nblocks{
        f.seek(SeekFrom::Start(CONT_SIZE + x * block_size));
        write_u64(f, 0)?;
        write_u64(f, block_size-16)?;
       }
       Ok({
        BlobStore { 
            file: ff, 
            hseed, 
            block_size, 
            nblocks,
            elems: 0 
        }
       })
    }

    pub fn open(fname:&str)-> Result<Self, BlobError>{
        let mut ff = OpenOptions::new().write(true).read(true).open(fname)?;
        let f = &mut ff ;
        f.seek(SeekFrom::Start(0))?;
        let hseed = read_u64(f)?;
        let block_size = read_u64(f)?;
        let nblocks = read_u64(f)?;
        let elems = read_u64(f)?;
        Ok( BlobStore { 
            file: ff, 
            hseed, 
            block_size, 
            nblocks, 
            elems 
        })
    }

    pub fn new_or_open(fname:&str, bsize:u64,nblocks:u64)->Result<Self, BlobError>{
        Self::new(fname,bsize,nblocks).or_else(|_|Self::open(fname))
    }

    pub fn inc_elems(&mut self, n:i32)->Result<(), BlobError>{
        if n > 0{
            self.elems += n as u64;
        }else{
            let n2 = (-n) as u64 ;
            if self.elems> n2{
                self.elems -= n2
            }
        }
        self.file.seek(SeekFrom::Start(24));
        write_u64(&mut self.file, self.elems);
        Ok(())
    }

    fn insert<K: Serialize, V: Serialize>(&mut self, k: K,v:V)->Result<(), BlobError>{
        self.remove(&k).ok() ;
        self.insert_only(k, v)
    }
    //does not remove if already tere
    pub fn insert_only<K:Serialize, V: Serialize>(&mut self, k:K, v:V)->Result<(), BlobError>{
        let blob = Blob::from(&k, &v)?;
        if blob.len()> self.block_size{
            //let the wrapper make a file with a bigger
            return  Err(BlobError::TooBig(blob.len()));
        }
        let bucket = blob.k_hash(self.hseed) % self.nblocks;
        let f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(CONT_SIZE + self.block_size * bucket))?;
        //start each loop at beginning of an elem
        //remeber klen == 0 means empty section
        loop{
            if pos >= CONT_SIZE + self.block_size * (bucket + 1){
                //reached end of data block
                //consider other handlings but this willtell the wrapper to make space
                //another option is to overflow onto the end of the file
                return Err(BlobError::NoRoom);
            }
            let klen = read_u64(f)?;
            let vlen = read_u64(f)?;
            if klen == 0 &&blob.len() < vlen {
                f.seek(SeekFrom::Start(pos))?;
                blob.out(f)?;
                //add pointer immediatly after data ends
                write_u64(f, 0)?;
                write_u64(f, (vlen - blob.len()) - 16)?;
                self.inc_elems(1)?;
                return Ok(());
            }
            pos = f.seek(SeekFrom::Start(pos + 16 + klen + vlen))?;
        }
    }

    pub fn b_start(&self, b:u64)->u64{
        CONT_SIZE  +self.block_size * b
    }

    pub fn get <K:Serialize>(&mut self,k:&K)->Result<Blob, BlobError>{
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let b_start = self.b_start(bucket);
        let b_end = self.b_start(bucket +1);
        let  f = &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(b_start))?;
        loop{
            if pos >= b_end{
                //Result <Option> also possible
                return Err(BlobError::NotFound);
            }
            //lazy, for v_large optimize by not reading the blob, only the key
            let b = Blob::read(f)?;
            if b.key_match(&s_blob){
                return Ok(b);
            }
            pos += b.len();
        }
    }
    pub fn remove <K: Serialize>(&mut self, k: &K)->Result<(), BlobError>{
        let s_blob = Blob::from(k, &0)?;
        let bucket = s_blob.k_hash(self.hseed) % self.nblocks;
        let b_start = self.b_start(bucket);
        let b_end = self.b_start (bucket + 1);
        let f= &mut self.file;
        let mut pos = f.seek(SeekFrom::Start(b_start))?;
        loop {
            if pos >= b_end{
                return Ok(());
            }
            let b = Blob::read(f)?;
            if b.key_match(&s_blob){
                //item found 
                let l = b.len();
                //if next block is empty
                if pos+l < b_end{
                    if read_u64(f)? == 0{
                        let nlen = read_u64(f)?;
                        f.seek(SeekFrom::Start(pos))?;
                        write_u64(f, 0)?;//klen 0 is empty
                        write_u64(f, l + nlen + 16)?;
                        return Ok(());
                    }
                }
                
                f.seek(SeekFrom::Start(pos))?;
                write_u64(f, 0)?;
                write_u64(f, l- 16)?;
                self.inc_elems(-1)?;
                return Ok(());
            }
            pos = f.seek(SeekFrom::Start(pos +b.len()))?;
        }
    }
} 

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    pub fn test_create_file(){
        let fs = "test_data/create_file";
        std::fs::remove_file(fs).ok();
        let mut bs = BlobStore::new(fs,1000,10).unwrap();
        let block_size = bs.block_size;
        let mut b2 = BlobStore::open(fs).unwrap();
        assert_eq!(b2.block_size,block_size);

        b2.insert_only("fish", "so long and thanks for for all teh fish").unwrap();
        b2.insert_only(23,  "a big number for small counters").unwrap();
        b2.insert_only("green", "is a color i guess").unwrap();
        b2.insert_only("happy", "is friend with sleepy").unwrap();

        drop(b2);
        let mut b3 = BlobStore::open(fs).unwrap();
        assert_eq!(b3.get(&"green").unwrap().get_v::<String>().unwrap(), "is a color i guess".to_string());
        b3.remove(&"green").ok();
        assert!(b3.get(&"green").is_err());
        assert!(b3.get(&"fish").is_ok())
    }
}

