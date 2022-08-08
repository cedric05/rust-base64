use core::panic;

#[inline]
fn get_base64_char(int: u8) -> char {
    match int {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        52 => '0',
        53 => '1',
        54 => '2',
        55 => '3',
        56 => '4',
        57 => '5',
        58 => '6',
        59 => '7',
        60 => '8',
        61 => '9',
        62 => '+',
        63 => '/',
        32 => 'g',
        48 => 'w',
        33 => 'h',
        34 => 'i',
        35 => 'j',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        26 => 'a',
        27 => 'b',
        28 => 'c',
        29 => 'd',
        30 => 'e',
        31 => 'f',
        49 => 'x',
        50 => 'y',
        51 => 'z',
        36 => 'k',
        37 => 'l',
        38 => 'm',
        39 => 'n',
        40 => 'o',
        41 => 'p',
        42 => 'q',
        43 => 'r',
        44 => 's',
        45 => 't',
        46 => 'u',
        47 => 'v',
        64 => '=',
        65_u8..=u8::MAX => {
            panic!()
        }
    }
}
pub fn base64_encode(array: &[u8]) -> String {
    array
        .chunks(3)
        .map(|x| match x.len() {
            3 => {
                let i1 = x[0] >> 2;
                let i2 = (x[1] >> 4) + ((x[0] << 6) >> 2);
                let i3 = ((x[1] & 0b0000_1111) << 2) + (x[2] >> 6);
                let i4 = x[2] & 0b0011_1111;
                (i1, i2, i3, i4)
            }
            2 => {
                let i1 = x[0] >> 2;
                let i2 = (x[1] >> 4) + ((x[0] << 6) >> 2);
                let i3 = (x[1] & 0b0000_1111) << 2;
                let i4 = 64;
                (i1, i2, i3, i4)
            }
            1 => {
                let i1 = x[0] >> 2;
                let i2 = (x[0] << 6) >> 2;
                let i3 = 64;
                let i4 = 64;
                (i1, i2, i3, i4)
            }
            _ => {
                panic!()
            }
        })
        .map(|(i, i2, i3, i4)| {
            [
                get_base64_char(i),
                get_base64_char(i2),
                get_base64_char(i3),
                get_base64_char(i4),
            ]
        })
        .flatten()
        .collect()
}

#[cfg(test)]
mod test {
    use crate::base64_encode;

    #[test]
    pub fn test_basic() {
        let a = 'a' as u8;
        assert_eq!("YQ==", base64_encode(&[a]));
    }
    #[test]
    pub fn test_basic2() {
        let a = 'a' as u8;
        assert_eq!("YWFh", base64_encode(&[a, a, a]));
    }
    #[test]
    pub fn test_basic3() {
        let a = 'a' as u8;
        assert_eq!("YWE=", base64_encode(&[a, a]));
    }

    #[test]
    pub fn test_basic4() {
        let a = "test";
        assert_eq!("dGVzdA==", base64_encode(a.as_bytes()));
    }

    #[test]
    pub fn test_basic5() {
        let a = "test";
        assert_eq!("dGVzdA==", base64_encode(a.as_bytes()));
    }

    #[test]
    pub fn test_base6() {
        assert_eq!(
            "d3d3LnB5dGhvbi5vcmc=",
            base64_encode("www.python.org".as_bytes())
        )
    }

    #[test]
    pub fn test_base7() {
        assert_eq!("AA==", base64_encode(&[0x00]))
    }
    #[test]
    pub fn test_base8() {
        assert_eq!("01a+b/cd", base64_encode(b"\xd3V\xbeo\xf7\x1d"))
    }
}
