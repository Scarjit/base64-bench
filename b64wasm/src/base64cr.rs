#![forbid(unsafe_code)]
extern crate alloc;
use alloc::vec::Vec;
use wasm_bindgen::prelude::*;

macro_rules! seq4 {
    [$($e:expr),*] => { [$($e,$e,$e,$e,)*] }
}

macro_rules! rep4 {
    [$($e:expr),*] => { [$($e,)*$($e,)*$($e,)*$($e,)*] }
}

use constants::{D0, D1, D2, D3, E0, E1, E2};

mod constants {
    pub(crate) static E0: [char; 256] = seq4![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];

    pub(crate) static E1: [char; 256] = rep4![
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];

    pub(crate) static E2: [char; 256] = E1;

    const FF: u32 = 33554431;

    pub(crate) static D0: [u32; 256] = [
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, 248, FF,
        FF, FF, 252, 208, 212, 216, 220, 224, 228, 232, 236, 240, 244, FF, FF, FF, FF, FF, FF, FF,
        0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60, 64, 68, 72, 76, 80, 84, 88,
        92, 96, 100, FF, FF, FF, FF, FF, FF, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140, 144,
        148, 152, 156, 160, 164, 168, 172, 176, 180, 184, 188, 192, 196, 200, 204, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
    ];

    pub(crate) static D1: [u32; 256] = [
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, 57347, FF,
        FF, FF, 61443, 16387, 20483, 24579, 28675, 32771, 36867, 40963, 45059, 49155, 53251, FF,
        FF, FF, FF, FF, FF, FF, 0, 4096, 8192, 12288, 16384, 20480, 24576, 28672, 32768, 36864,
        40960, 45056, 49152, 53248, 57344, 61440, 1, 4097, 8193, 12289, 16385, 20481, 24577, 28673,
        32769, 36865, FF, FF, FF, FF, FF, FF, 40961, 45057, 49153, 53249, 57345, 61441, 2, 4098,
        8194, 12290, 16386, 20482, 24578, 28674, 32770, 36866, 40962, 45058, 49154, 53250, 57346,
        61442, 3, 4099, 8195, 12291, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF,
    ];

    pub(crate) static D2: [u32; 256] = [
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, 8392448,
        FF, FF, FF, 12586752, 3328, 4197632, 8391936, 12586240, 3584, 4197888, 8392192, 12586496,
        3840, 4198144, FF, FF, FF, FF, FF, FF, FF, 0, 4194304, 8388608, 12582912, 256, 4194560,
        8388864, 12583168, 512, 4194816, 8389120, 12583424, 768, 4195072, 8389376, 12583680, 1024,
        4195328, 8389632, 12583936, 1280, 4195584, 8389888, 12584192, 1536, 4195840, FF, FF, FF,
        FF, FF, FF, 8390144, 12584448, 1792, 4196096, 8390400, 12584704, 2048, 4196352, 8390656,
        12584960, 2304, 4196608, 8390912, 12585216, 2560, 4196864, 8391168, 12585472, 2816,
        4197120, 8391424, 12585728, 3072, 4197376, 8391680, 12585984, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
    ];

    pub(crate) static D3: [u32; 256] = [
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, 4063232,
        FF, FF, FF, 4128768, 3407872, 3473408, 3538944, 3604480, 3670016, 3735552, 3801088,
        3866624, 3932160, 3997696, FF, FF, FF, FF, FF, FF, FF, 0, 65536, 131072, 196608, 262144,
        327680, 393216, 458752, 524288, 589824, 655360, 720896, 786432, 851968, 917504, 983040,
        1048576, 1114112, 1179648, 1245184, 1310720, 1376256, 1441792, 1507328, 1572864, 1638400,
        FF, FF, FF, FF, FF, FF, 1703936, 1769472, 1835008, 1900544, 1966080, 2031616, 2097152,
        2162688, 2228224, 2293760, 2359296, 2424832, 2490368, 2555904, 2621440, 2686976, 2752512,
        2818048, 2883584, 2949120, 3014656, 3080192, 3145728, 3211264, 3276800, 3342336, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
        FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF, FF,
    ];
}

pub trait Base64 {
    fn encode(&self) -> Vec<u8>;
    fn decode(&self) -> Vec<u8>;
}

impl Base64 for [u8] {
    fn encode(&self) -> Vec<u8> {
        let len = self.len();

        let mut dest = vec![0u8; ((4 * len / 3) + 3) & !3];

        let mut i = 0;
        let mut j = 0;

        if len > 2 {
            while i < len - 2 {
                let t1 = self[i];
                let t2 = self[i + 1];
                let t3 = self[i + 2];

                dest[j] = E0[t1 as usize] as u8;
                dest[j + 1] = E1[(((t1 & 0x03) << 4) | ((t2 >> 4) & 0x0F)) as usize] as u8;
                dest[j + 2] = E1[(((t2 & 0x0F) << 2) | ((t3 >> 6) & 0x03)) as usize] as u8;
                dest[j + 3] = E2[t3 as usize] as u8;

                i += 3;
                j += 4;
            }
        }
        match len - i {
            0 => {}
            1 => {
                let t1 = self[i];

                dest[j] = E0[t1 as usize] as u8;
                dest[j + 1] = E1[((t1 & 0x03) << 4) as usize] as u8;
                dest[j + 2] = b'=';
                dest[j + 3] = b'=';
            }
            _ => {
                /* case 2 */
                let t1 = self[i] as usize;
                let t2 = self[i + 1] as usize;

                dest[j] = E0[t1] as u8;
                dest[j + 1] = E1[((t1 & 0x03) << 4) | ((t2 >> 4) & 0x0F)] as u8;
                dest[j + 2] = E2[(t2 & 0x0F) << 2] as u8;
                dest[j + 3] = b'=';
            }
        }

        dest
    }

    fn decode(&self) -> Vec<u8> {
        let mut len = self.len();

        if self[len - 1] == b'=' {
            len -= 1;
            if self[len - 1] == b'=' {
                len -= 1;
            }
        }

        let mut dest = vec![0u8; (3 * (self.len() / 4)) - (self.len() - len)];


        let leftover = len % 4;
        let chunks = if leftover == 0 { len / 4 - 1 } else { len / 4 };

        let mut j = 0;
        let mut k = 0;
        for _ in 0..chunks {
            let x: u32 = D0[self[k] as usize]
                | D1[self[1 + k] as usize]
                | D2[self[2 + k] as usize]
                | D3[self[3 + k] as usize];

            dest[j] = x as u8;
            dest[j + 1] = (x >> 8) as u8;
            dest[j + 2] = (x >> 16) as u8;

            j += 3;
            k += 4;
        }

        match leftover {
            0 => {
                let x: u32 = D0[self[k] as usize]
                    | D1[self[1 + k] as usize]
                    | D2[self[2 + k] as usize]
                    | D3[self[3 + k] as usize];

                dest[j] = x as u8;
                dest[j + 1] = (x >> 8) as u8;
                dest[j + 2] = (x >> 16) as u8;

                // (chunks + 1) * 3)
                return dest
            }
            1 => {
                /* with padding this is an impossible case */
                let x: u32 = D0[self[k] as usize]; // i.e. first char/byte in int
                dest[j] = x as u8;
            }
            2 => {
                // * case 2, 1  output byte */
                let x: u32 = D0[self[k] as usize] | D1[self[1 + k] as usize]; // i.e. first char
                dest[j] = x as u8;
            }
            _ => {
                let x: u32 =
                    D0[self[k] as usize] | D1[self[1 + k] as usize] | D2[self[2 + k] as usize]; /* 0x3c */
                dest[j] = x as u8;
                dest[j + 1] = (x >> 8) as u8;
            }
        }

        // 3 * chunks + (6 * leftover) / 8
        dest
    }
}

#[wasm_bindgen]
pub fn encode4(input: &str) -> String {
    let x = input.as_bytes().encode();
    String::from_utf8(x).unwrap()
}

#[wasm_bindgen]
pub fn decode4(input: &str) -> Result<String, JsValue> {
    let x = input.as_bytes().decode();
    match String::from_utf8(x) {
        Ok(s) => Ok(s),
        Err(e) => Err(JsValue::from_str(&format!("{}", e))),
    }
}

