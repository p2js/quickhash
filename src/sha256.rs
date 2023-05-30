use crate::HashResult;

//keys/constants
const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5, 
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

pub fn sha256(source: &Vec<u8>) -> HashResult {
    let mut h: [u32; 8] = [
        0x6a09e667, 
        0xbb67ae85, 
        0x3c6ef372, 
        0xa54ff53a, 
        0x510e527f, 
        0x9b05688c, 
        0x1f83d9ab,
        0x5be0cd19
    ]; //initialised hash values

    //message padding
    let mut message = source.clone();

    //append 0b10000000 then 0s to the source until (length mod 64) == 56
    message.push(0b10000000);
    while (message.len() % 64) != 56 {
        message.push(0);
    }

    //get source length as 64 bit unsigned integer
    //*8 for bit length, not byte length
    let l = source.len() as u64 * 8; 

    //append source length (big endian)
    message.extend_from_slice(&(l).to_be_bytes());

    //for every 512-bit (64 byte) chunk
    for chunk in message.chunks_exact(64) {
        //create 64 entry array of 32 bit words
        let mut w: [u32; 64] = [0; 64];
        //copy chunk into the first 16 words
        for i in 0..16 {
            let word = &chunk[i*4..(i+1)*4];
            w[i] = u32::from_be_bytes(word.try_into().unwrap());
        }
        //Extend the first 16 words into the remaining 48 words
        for i in 16..64 {
            let s0 = w[i-15].rotate_right(7) ^ w[i-15].rotate_right(18) ^ (w[i-15] >> 3);
            let s1 = w[i-2].rotate_right(17) ^ w[i-2].rotate_right(19) ^ (w[i-2] >> 10);
            w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
        }

        //Initialize working variables to current hash value
        let mut temp = h;
        //compression (main loop)
        for i in 0..64 {
            let s1 = (temp[4].rotate_right(6)) ^ (temp[4].rotate_right(11)) ^ (temp[4].rotate_right(25));
            let ch = (temp[4] & temp[5]) ^ ((!temp[4]) & temp[6]);
            let temp1 = temp[7].wrapping_add(s1).wrapping_add(ch).wrapping_add(K[i]).wrapping_add(w[i]);
            let s0 = (temp[0].rotate_right(2)) ^ (temp[0].rotate_right(13)) ^ (temp[0].rotate_right(22));
            let maj = (temp[0] & temp[1]) ^ (temp[0] & temp[2]) ^ (temp[1] & temp[2]);
            let temp2 = s0.wrapping_add(maj);
    
            temp[7] = temp[6];
            temp[6] = temp[5];
            temp[5] = temp[4];
            temp[4] = temp[3].wrapping_add(temp1);
            temp[3] = temp[2];
            temp[2] = temp[1];
            temp[1] = temp[0];
            temp[0] = temp1.wrapping_add(temp2);
        }
        //add compressed chunk to current hash value
        for i in 0..8 {
            h[i] = h[i].wrapping_add(temp[i]);
        }
    };

    //produce hash result by appending all the hash values as [u8; 4]
    let mut hash_result = [0; 32];
    for i in 0..8 {
        hash_result[i*4..(i+1)*4].copy_from_slice(&h[i].to_be_bytes());
    }

    HashResult::SHA256(hash_result)
}
