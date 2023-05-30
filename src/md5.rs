use crate::HashResult;

//hash keys/constants
const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391
];

//per-round shift amounts
const S: [u32; 64] = [
    07, 12, 17, 22, 07, 12, 17, 22, 07, 12, 17, 22, 07, 12, 17, 22,
    05, 09, 14, 20, 05, 09, 14, 20, 05, 09, 14, 20, 05, 09, 14, 20,
    04, 11, 16, 23, 04, 11, 16, 23, 04, 11, 16, 23, 04, 11, 16, 23,
    06, 10, 15, 21, 06, 10, 15, 21, 06, 10, 15, 21, 06, 10, 15, 21
];

pub fn md5(source: &Vec<u8>) -> HashResult {
    let mut h: [u32; 4] = [
        0x67452301,
        0xefcdab89,
        0x98badcfe,
        0x10325476       
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

    //append source length (little endian)
    message.extend_from_slice(&(l).to_le_bytes());

    for chunk in message.chunks_exact(64) {
        //break chunk into 16 32-bit words
        let mut m: [u32; 16] = [0; 16];
        for i in 0..16 {
            let word = &chunk[i*4..(i+1)*4];
            m[i] = u32::from_le_bytes(word.try_into().unwrap());
        }

        //initialise working variables to current hash value
        let mut temp = h;
        //compression (main loop)
        for i in 0usize..64usize {
            let (mut temp1, temp2) = match i {
                0..=15 => {
                    ( 
                        (temp[1] & temp[2]) | (!temp[1] & temp[3]),
                        i
                    )
                }
                16..=31 => {
                    (
                        (temp[3] & temp[1]) | (!temp[3] & temp[2]),
                        (5*i).wrapping_add(1) % 16
                    )
                }
                32..=47 => {
                    (
                        temp[1] ^ temp[2] ^ temp[3],
                        (3*i).wrapping_add(5) % 16
                    )
                }
                48..=63 => {
                    (
                        temp[2] ^ (temp[1] | !temp[3]),
                        (7*i) % 16
                    )
                }
                _ => unreachable!()
            };

            temp1 = temp1.wrapping_add(temp[0]).wrapping_add(K[i]).wrapping_add(m[temp2]);
            temp[0] = temp[3];
            temp[3] = temp[2];
            temp[2] = temp[1];
            temp[1] = temp[1].wrapping_add(temp1.rotate_left(S[i]));
        }

        for i in 0..4 {
            h[i] = h[i].wrapping_add(temp[i]);
        }
    }

    //produce hash result by appending all the hash values as [u8; 4]
    let mut hash_result = [0; 16];
        for i in 0..4 {
        hash_result[i*4..(i+1)*4].copy_from_slice(&h[i].to_le_bytes());
    }

    HashResult::MD5(hash_result)
}
 