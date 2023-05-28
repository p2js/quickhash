use crate::HashResult;

pub fn md5(source: &Vec<u8>) -> HashResult {
    HashResult::MD5([0; 16])
}