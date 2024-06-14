use num_bigint::BigUint;
use ark_std::io::{Read, Seek};

use crate::bin_utils::BinFile;


pub struct ZKeyUtils {
    pub file_path: String,
}

impl ZKeyUtils {
    pub fn new(file_path: String) -> ZKeyUtils {
        ZKeyUtils {
            file_path
        }
    }

    pub fn load_fflonk_zkey_header<R: Read + Seek>(mut reader: R) {

    }
}

pub const ZKEY_FF_HEADER_SECTION: u32 = 2;
pub const ZKEY_FF_ADDITIONS_SECTION: u32 = 3;
pub const ZKEY_FF_A_MAP_SECTION: u32 = 4;
pub const ZKEY_FF_B_MAP_SECTION: u32 = 5;
pub const ZKEY_FF_C_MAP_SECTION: u32 = 6;
pub const ZKEY_FF_QL_SECTION: u32 = 7;
pub const ZKEY_FF_QR_SECTION: u32 = 8;
pub const ZKEY_FF_QM_SECTION: u32 = 9;
pub const ZKEY_FF_QO_SECTION: u32 = 10;
pub const ZKEY_FF_QC_SECTION: u32 = 11;
pub const ZKEY_FF_SIGMA1_SECTION: u32 = 12;
pub const ZKEY_FF_SIGMA2_SECTION: u32 = 13;
pub const ZKEY_FF_SIGMA3_SECTION: u32 = 14;
pub const ZKEY_FF_LAGRANGE_SECTION: u32 = 15;
pub const ZKEY_FF_PTAU_SECTION: u32 = 16;
pub const ZKEY_FF_C0_SECTION: u32 = 17;

// zkey_fflonk.hpp
pub struct FflonkZkeyHeader {
    pub protocol_id: u32,
    pub q_prime: BigUint,
    pub n8q: u32,
    pub r_prime: BigUint,

    pub n_vars: u32,
    pub n_public: u32,
    pub domain_size: u32,
    pub n_additions: u32,
    pub n_constraints: u32,

    // void *k1;
    // void *k2;
    // void *w3;
    // void *w4;
    // void *w8;
    // void *wr;
    // void *X2;
    // void *C0;
}

impl FflonkZkeyHeader {
    pub fn load_fflonk_zkey_header<R: Read + Seek>(bin_file: &mut BinFile<R>) {
        bin_file.start_read_section(ZKEY_FF_HEADER_SECTION, None).unwrap();
        

    }

    // pub fn bin_read_section(mut reader: R, section: i32) {

    // }
}


#[cfg(test)]
mod test {
    use std::fs::File;

    use crate::bin_utils::BinFile;

    use super::{FflonkZkeyHeader};

    #[test]
    fn test_load_header() {

        let zkey_file = "./test-vectors/simple.r1cs".to_string();
        let reader = File::open(zkey_file).unwrap();
        let mut bin_file = BinFile::new_from_reader(reader, "ZKEY".to_string(), 1).unwrap();
        FflonkZkeyHeader::load_fflonk_zkey_header(&mut bin_file);
    }

}