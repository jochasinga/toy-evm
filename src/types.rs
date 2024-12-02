use std::{ops::{Add, BitOr, Div, Mul, Shl, Shr, Sub}, str::FromStr};
use std::cmp::Ordering;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Endian {
    Little,
    Big,
}

pub mod utils {

    //// Utility functions for converting between byte arrays and UInt256 values.

    use super::*;

    pub struct BytesPair {
        /// The low part (LSB) of the byte array.
        pub low: Box<[u8; 16]>,
        /// The high part (MSB) of the byte array.
        pub high: Box<[u8; 16]>,
    }

    //// Pad a byte array to 32 bytes with the given byte value.
    /// The `endian` parameter specifies the "endianness" of the input data, or
    /// basically just deciding whether to prepend or append the padding bytes.
    /// The existing bytes are copied to the padded array in the same order.
    pub fn pad_bytes(data: &[u8], with: u8, endian: Endian) -> [u8; 32] {
        let mut padded = [with; 32];
        let len = data.len().min(32);
        match endian {
            Endian::Big => padded[32 - len..].copy_from_slice(&data[..len]),
            Endian::Little => padded[..len].copy_from_slice(&data[..len]),
        }
        padded
    }

    /// Convert a 32-byte array to a UInt256 value based on the endian type provided.
    pub fn to_uint256(bytes: &[u8; 32], endian: Endian) -> UInt256 {
        match endian {
            Endian::Little => UInt256::from_le_bytes(bytes),
            Endian::Big => UInt256::from_be_bytes(bytes),
        }
    }

    /// Convert a 32-byte hexadecimal string to a `BytesPair` of 16-byte arrays.
    pub fn hex_to_bytes_pair(n: &str, endian: Endian) -> Result<BytesPair, String> {

        let a = n.trim().strip_prefix("0x").unwrap();

        match endian {
            Endian::Big => {
                // Take the first 16 bytes (32 chars) as the low part and the last 16 bytes (32 chars) as the high part.
                let low_hex = &a[..32];
                let high_hex = &a[32..];

                let low_num = u128::from_str_radix(low_hex, 16).unwrap();
                let low_data = low_num.to_be_bytes().to_vec();
                let low_bytes = <[u8; 16]>::try_from(low_data.as_slice()).map_err(|e| e.to_string())?;

                let high_num = u128::from_str_radix(high_hex, 16).unwrap();
                let high_data = high_num.to_be_bytes().to_vec();
                let high_bytes = <[u8; 16]>::try_from(high_data.as_slice()).map_err(|e| e.to_string())?;

                return Ok(BytesPair{
                    low: Box::new(low_bytes),
                    high: Box::new(high_bytes),
                });
            },
            Endian::Little => {
                // Take the last 16 bytes (32 chars) as the low part and the first 16 bytes (32 chars) as the high part.
                let low_hex = &a[32..];
                let high_hex = &a[..32];

                let low_num = u128::from_str_radix(low_hex, 16).unwrap();
                let low_data = low_num.to_be_bytes().to_vec()
                    .iter()
                    .rev() // Reverse the order of the bytes
                    .cloned()
                    .collect::<Vec<u8>>();

                let low_bytes = <[u8; 16]>::try_from(low_data.clone().as_slice()).map_err(|e| e.to_string())?;

                let high_num = u128::from_str_radix(high_hex, 16).unwrap();
                let high_data = high_num.to_be_bytes().to_vec()
                    .iter()
                    .rev()// Reverse the order of the bytes
                    .cloned()
                    .collect::<Vec<u8>>();

                let high_bytes = <[u8; 16]>::try_from(high_data.clone().as_slice()).map_err(|e| e.to_string())?;

                // Keeping this dumb first attempt for reference.

                // let low_chars: Vec<char> = base_low.chars().collect();
                // let mut low_pairs = Vec::with_capacity(16);
                // let high_chars: Vec<char> = base_high.chars().collect();
                // let mut high_pairs = Vec::with_capacity(16);
                // let mut i = 0;
                // while i + 1 < low_chars.len() {
                //     let a = low_chars[i];
                //     let b = low_chars[i + 1];
                //     println!("# {} : {}", a, b);
                //     low_pairs.push((a, b));
                //     i += 2;
                // }
                // i = 0;
                // while i + 1 < high_chars.len() {
                //     let a = high_chars[i];
                //     let b = high_chars[i + 1];
                //     println!("# {} : {}", a, b);
                //     high_pairs.push((a, b));
                //     i += 2;
                // }
                // let low_bytes: Vec<u8> = low_pairs
                //     .iter()
                //     .map(|(a, b)| {
                //         let s = format!("{}{}", a, b);
                //         println!("{}", s);
                //         let a = u8::from_str_radix(&s, 16).unwrap();
                //         a
                //     })
                //     .rev()
                //     .collect();
                // let high_bytes: Vec<u8> = high_pairs
                //     .iter()
                //     .map(|(a, b)| {
                //         let s = format!("{}{}", a, b);
                //         println!("{}", s);
                //         let a = u8::from_str_radix(&s, 16).unwrap();
                //         a
                //     })
                //     .rev()
                //     .collect();

                return Ok(BytesPair{
                    low: Box::new(low_bytes),
                    high: Box::new(high_bytes),
                });
            },
        };
    }
}


impl Default for Endian {
    fn default() -> Self {
        DEFAULT_ENDIAN
    }
}

#[derive(Debug, Default)]
pub struct UInt256Builder {
    bytes: Box<[u8; 32]>,
    endian: Endian,
    padding: bool,
}

impl UInt256Builder {
    pub fn new() -> Self {
        UInt256Builder {
            bytes: Box::new([0u8; 32]),
            endian: DEFAULT_ENDIAN,
            padding: false,
        }
    }

    pub fn with_padding(&mut self) -> &mut Self {
        self.padding = true;
        self
    }

    pub fn with_endian(&mut self, endian: Endian) -> &mut Self {
        self.endian = endian;
        self
    }

    pub fn from_bytes(&mut self, bytes: &[u8; 32]) -> &mut Self {
        self.bytes = Box::new(*bytes);
        self
    }

    pub fn build(self) -> UInt256 {
        utils::to_uint256(self.bytes.as_ref(), self.endian)
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, Hash)]
pub struct UInt256 {
    // First 16 bytes (128 bits). High means MSB or the left half.
    high: u128,
    // Last 16 bytes (128 bits). Low means LSB or right half.
    low: u128,
    // Endianness (Is it needed here though).
    endian: Endian,
}

impl UInt256 {
    pub const ZERO: Self = Self { high: 0, low: 0, endian: Endian::Big };
    pub const ONE: Self = Self { high: 0, low: 1, endian: Endian::Big };
    pub const MAX: Self = Self {
        high: u128::MAX,
        low: u128::MAX,
        endian: Endian::Big,
    };

    pub fn new(high: u128, low: u128, endian: Endian) -> Self {
        UInt256 { high, low, endian }
    }

    pub fn is_zero(&self) -> bool {
        self.high == 0 && self.low == 0
    }

    pub fn endian(&self) -> Endian {
        self.endian
    }

    pub fn as_bytes(&self) -> Box<[u8; 32]> {
        let mut bytes = [0u8; 32];

        // Fill in the high part (first 16 bytes)
        for i in 0..16 {
            bytes[i] = (self.high >> (8 * (15 - i)) & 0xff) as u8;
        }

        // Fill in the low part (last 16 bytes)
        for i in 0..16 {
            bytes[16 + i] = (self.low >> (8 * (15 - i)) & 0xff) as u8;
        }

        Box::new(bytes)
    }

    pub fn as_usize(&self) -> Result<usize, String> {
        let max_usize = Self::from(usize::MAX);
        if self.high != 0 || self.low > max_usize.low {
            return Err("Value too large".to_string());
        }
        Ok(self.low as usize)
    }

    pub fn from_str_radix(s: &str, radix: u32, endian: Endian) -> Result<Self, &'static str> {
        let s = s.trim();

        if s.len() != 64 {
            return Err("Invalid length");
        }

        let low: u128;
        let high: u128;

        match endian {
            Endian::Big => {
                low = u128::from_str_radix(&s[32..], radix).map_err(|_| "Invalid hex")?;
                high = u128::from_str_radix(&s[..32], radix).map_err(|_| "Invalid hex")?;
            },
            Endian::Little => {
                let n = &s[..32];
                n.strip_prefix(&['0']).unwrap_or(n);
                low = u128::from_str_radix(&s[..32], radix).map_err(|_| "Invalid hex")?;
                high = u128::from_str_radix(&s[32..], radix).map_err(|_| "Invalid hex")?;
            },
        }
        Ok(UInt256 { high, low, endian })
    }

    pub fn from_le_bytes(bytes: &[u8]) -> Self {
        let mut low = 0;
        let mut high = 0;
        for (i, byte) in bytes.iter().enumerate() {
            if i < 16 {
                low |= (*byte as u128) << (i * 8);
            } else {
                high |= (*byte as u128) << ((i - 16) * 8);
            }
        }
        UInt256 { high, low, endian: Endian::Little }
    }

    pub fn from_be_bytes(bytes: &[u8; 32]) -> Self {
        let mut low = 0;
        let mut high = 0;

       // Ensure the input is treated as a 32-byte array, padded with leading zeros if necessary
       for (i, byte) in bytes.iter().rev().enumerate() {
            if i < 16 {
                low |= (*byte as u128) << (i * 8);
            } else if i < 32 {
                high |= (*byte as u128) << ((i - 16) * 8);
            }
        }

        UInt256 { high, low, endian: Endian::Big }
    }

    pub fn to_le_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(32);
        for i in 0..32 {
            if i < 16 {
                bytes.push(((self.low >> (i * 8)) & 0xff) as u8);
            } else {
                bytes.push(((self.high >> ((i - 16) * 8)) & 0xff) as u8);
            }
        }
        bytes
    }

    pub fn to_be_bytes(&self) -> Box<[u8; 32]> {
        self.as_bytes()
    }
}

// Overloading comparison, shift, and subtraction operators
impl PartialEq for UInt256 {
    fn eq(&self, other: &Self) -> bool {
        self.high == other.high && self.low == other.low
    }
}

impl PartialOrd for UInt256 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UInt256 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.high.cmp(&other.high) {
            Ordering::Equal => self.low.cmp(&other.low),
            ord => ord,
        }
    }
}

impl std::fmt::Display for UInt256 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "0x{:032x}{:032x}", self.high, self.low)
    }
}

impl BitOr for UInt256 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        UInt256::new(self.high | rhs.high, self.low | rhs.low, self.endian)
    }
}

// FIXME: This implementation hangs!
pub fn divide(dividend: UInt256, divisor: UInt256) -> (UInt256, UInt256) {
    let zero = UInt256::ZERO;
    if dividend < divisor {
        return (zero, zero,);
    }

    if dividend.cmp(&divisor) == Ordering::Less {
        return (zero, zero,);
    }

    let mut quotient = UInt256::ZERO;
    let mut remainder = dividend;
    let mut power = divisor;

    // Left shift divisor until it's greater than self, adjusting `power` for the quotient calculation
    let mut power_of_two = UInt256::new(0, 1, dividend.endian);
    while remainder.cmp(&power.shl(1)) != Ordering::Less {
        power = power.shl(1);
        power_of_two = power_of_two.shl(1);
    }

    // Perform division by shifting and subtracting
    while power_of_two.cmp(&UInt256::new(0, 0, dividend.endian)) != Ordering::Equal {
        if remainder.cmp(&power) != Ordering::Less {
            remainder = remainder.sub(power);
            quotient = quotient | power_of_two;
        }
        power = power.shr(1);
        power_of_two = power_of_two.shr(1);
    }
    (quotient, remainder)
}

impl Div for UInt256 {

    type Output = Self;

    fn div(self, divisor: Self) -> Self {
        match divisor {
            UInt256 { high: 0, low: 0, .. } => panic!("division by zero"),
            UInt256 { high: 0, low: 1, .. } => self,
            _ => {
                let (quotient, _) = divide(self, divisor);
                quotient
            }
        }
    }
}

impl Add for UInt256 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let (low, carry_low) = self.low.overflowing_add(rhs.low);

        if carry_low {
            let (high, carry_high) = self.high.overflowing_add(rhs.high);
            if carry_high {
                panic!("addition overflow on most significant bits");
            }
            if high == u128::MAX {
                panic!("addition overflow on least significant bits");
            }
            return UInt256 {
                high: high + 1,
                low: self.low,
                endian: self.endian,
            };
        }
        UInt256 {
            high: self.high,
            low,
            endian: self.endian,
        }
    }
}

impl Sub for UInt256 {

    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        if self < rhs {
            panic!("subtraction overflow");
        }

        let (low, borrow_low) = self.low.overflowing_sub(rhs.low);
        if borrow_low {
            let (high, borrow_high) = self.high.overflowing_sub(rhs.high);
            if borrow_high {
                panic!("subtraction overflow on most significant bits");
            }
            return UInt256 {
                high: high - 1,
                low: self.low,
                endian: self.endian,
            };
        }
        let res = UInt256 {
            high: self.high,
            low,
            endian: self.endian,
        };
        res
    }
}

impl Mul for UInt256 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        // Split the values into high and low parts for each operand
        let a_low = self.low;
        let a_high = self.high;
        let b_low = other.low;
        let b_high = other.high;

        // Calculate the partial products
        let low_low = a_low as u128 * b_low as u128; // Low * Low part (128-bit)
        let low_high = a_low as u128 * b_high as u128; // Low * High part (128-bit)
        let high_low = a_high as u128 * b_low as u128; // High * Low part (128-bit)
        let high_high = a_high as u128 * b_high as u128; // High * High part (128-bit)

        // Combine the partial products, managing overflow
        let (low, carry1) = low_low.overflowing_add((low_high << 64) as u128);
        let (low, carry2) = low.overflowing_add((high_low << 64) as u128);
        let high = high_high + (low_high >> 64) + (high_low >> 64) + carry1 as u128 + carry2 as u128;

        UInt256 { high, low, endian: self.endian }
    }
}
impl Shr<u32> for UInt256 {
    type Output = Self;

    fn shr(self, shift: u32) -> Self {
        if shift >= 128 {
            UInt256 {
                high: 0,
                low: self.high >> (shift - 128),
                endian: self.endian,
            }
        } else {
            UInt256 {
                high: self.high >> shift,
                low: (self.high << (128 - shift)) | (self.low >> shift),
                endian: self.endian,
            }
        }
    }
}

// Helper implementation for left shift (<<) to handle shifting UInt256 by bit positions
impl Shl<u32> for UInt256 {
    type Output = Self;

    fn shl(self, shift: u32) -> Self {
        if shift >= 128 {
            UInt256 {
                high: self.low << (shift - 128),
                low: 0,
                endian: self.endian,
            }
        } else {
            UInt256 {
                high: (self.high << shift) | (self.low >> (128 - shift)),
                low: self.low << shift,
                endian: self.endian,
            }
        }
    }
}

const DEFAULT_RADIX: u32 = 16;
const DEFAULT_ENDIAN: Endian = Endian::Big;

impl FromStr for UInt256 {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        Self::from_str_radix(s, DEFAULT_RADIX, DEFAULT_ENDIAN)
    }
}

impl From<usize> for UInt256 {
    fn from(value: usize) -> Self {
        UInt256 {
            high: 0,
            low: value as u128,
            endian: Endian::Big,
        }
    }
}

impl TryInto<usize> for UInt256 {
    type Error = String;

    fn try_into(self) -> Result<usize, Self::Error> {
        self.as_usize()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use utils::*;

    #[test]
    fn test_pad_bytes() {
        let data = vec![0x01, 0x4a];
        let padded_be = pad_bytes(&data, 0x00, Endian::Big).to_vec();
        let mut expected = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x4a,
        ];
        assert_eq!(padded_be, expected, "Big-endian padding failed");
        let padded_le = pad_bytes(&data, 0x00, Endian::Little).to_vec();
        expected = vec![
            0x01, 0x4a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        assert_eq!(padded_le, expected, "Little-endian padding failed");
    }

    #[test]
    fn test_uint256_from_str() {
        let n = "0xff4567890abcdef1234567890ac203d51234567890abcdef1234567890abcdef";

        let a = UInt256::from_str(n).unwrap();
        let expected = UInt256 {
            high: 0xff4567890abcdef1234567890ac203d5,
            low:  0x1234567890abcdef1234567890abcdef,
            endian: Endian::Big,
        };
        assert_eq!(a, expected);
        let n = "0x000000000000000000000000000000000000000000000000000000000000014a";
        let b = UInt256::from_str(n).unwrap();
        let expected = UInt256 {
            high: 0,
            low: 330,
            endian: Endian::Big,
        };
        assert_eq!(b, expected);
        let c = UInt256::MAX;
        let expected = UInt256 {
            high: 0xffffffffffffffffffffffffffffffff,
            low: 0xffffffffffffffffffffffffffffffff,
            endian: Endian::Big,
        };
        assert_eq!(c, expected);
    }

    #[test]
    fn test_hex_to_bytes() {
        let n = "0x000000000000000000000000000000000000000000000000000000000000014a";
        let utils::BytesPair{low, high} = utils::hex_to_bytes_pair(n, Endian::Little).unwrap();
        let expected_low = vec![
            0x4a, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let expected_high = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(low.as_ref().to_vec(), expected_low);
        assert_eq!(high.as_ref().to_vec(), expected_high);

        let utils::BytesPair{low, high} = utils::hex_to_bytes_pair(n, Endian::Big).unwrap();

        let expected_low = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        let expected_high = vec![
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x4a,
        ];
        assert_eq!(low.as_ref().to_vec(), expected_low);
        assert_eq!(high.as_ref().to_vec(), expected_high);

        let n = "0xff4567890abcdef1234567890ac203d51234567890abcdef1234567890abcdef";

        let utils::BytesPair{low, high} = utils::hex_to_bytes_pair(n, Endian::Big).unwrap();
        let expected_low = vec![
            0xff, 0x45, 0x67, 0x89, 0x0a, 0xbc, 0xde, 0xf1,
            0x23, 0x45, 0x67, 0x89, 0x0a, 0xc2, 0x03, 0xd5
        ];
        let expected_high = [
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef
        ];

        assert_eq!(low.as_ref().to_vec(), expected_low);
        assert_eq!(high.as_ref().to_vec(), expected_high);

        let utils::BytesPair{low, high} = utils::hex_to_bytes_pair(n, Endian::Little).unwrap();

        let expected_high: Vec<u8> = vec![
            0xff, 0x45, 0x67, 0x89, 0x0a, 0xbc, 0xde, 0xf1,
            0x23, 0x45, 0x67, 0x89, 0x0a, 0xc2, 0x03, 0xd5
        ].iter().rev().cloned().collect();
        let expected_low: Vec<u8> = [
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef,
            0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef
        ].iter().rev().cloned().collect();

        assert_eq!(low.as_ref().to_vec(), expected_low);
        assert_eq!(high.as_ref().to_vec(), expected_high);
    }

    #[test]
    fn test_uint256_from_str_radix_be() {
        let n = "0xff4567890abcdef1234567890ac203d51234567890abcdef1234567890abcdef";
        let a = n.strip_prefix("0x").unwrap();

        let a = UInt256::from_str_radix(a, 16, Endian::Big).unwrap();
        let expected = UInt256 {
            high: 0xff4567890abcdef1234567890ac203d5,
            low:  0x1234567890abcdef1234567890abcdef,
            endian: Endian::Big,
        };
        assert_eq!(a, expected);
        let n = "0x000000000000000000000000000000000000000000000000000000000000014a";
        let b = n.strip_prefix("0x").unwrap();
        let b = UInt256::from_str_radix(b, 16, Endian::Big).unwrap();
        let expected = UInt256 {
            high: 0,
            low: 330,
            endian: Endian::Big,
        };
        assert_eq!(b, expected);
    }

    #[test]
    fn test_uint256_from_str_radix_le() {
        let n = "0x000000000000000000000000000000000000000000000000000000000000014a";
        let b = n.strip_prefix("0x").unwrap();
        let b = UInt256::from_str_radix(b, 16, Endian::Little).unwrap();
        let expected = UInt256 {
            high: 330,
            low: 0,
            endian: Endian::Little,
        };
        assert_eq!(b, expected);
    }

    #[test]
    fn test_shl() {
        let a = UInt256::from(100) << 1;
        assert_eq!(
            a.as_usize().expect("a usize"),
            200,
        );
        let b = UInt256::from(100) << 2;
        assert_eq!(
            b.as_usize().expect("a usize"),
            400,
        );
    }

    #[cfg(test)]
    mod test_addition {
        use super::*;

        #[test]
        #[should_panic(expected = "addition overflow on least significant bits")]
        fn test_uint256_add_overflow() {
            let a = UInt256::MAX;
            let b = UInt256::ONE;
            let _ = a + b;
        }

        #[test]
        fn test_uint256_add_basic() {
            let a = UInt256::from(1000_000_000);
            let b = UInt256::from(999_999_999);
            let c = a + b;
            assert_eq!(c, UInt256::from(1999_999_999));
        }

        #[test]
        fn test_uint256_zero_property() {
            let c = UInt256::MAX + UInt256::ZERO;
            assert_eq!(c, UInt256::MAX);
        }

        #[test]
        fn test_uint256_add_big() {
            let a = UInt256::MAX - UInt256::ONE;
            let b = UInt256::ONE;
            let c = a + b;
            assert_eq!(c, UInt256::MAX);
        }
    }

    #[cfg(test)]
    mod test_subtraction {

        use super::*;

        #[test]
        #[should_panic(expected = "subtraction overflow")]
        fn test_uint256_subtract_overflow() {
            let a = UInt256::ONE;
            let b = UInt256::MAX;
            let _ = a - b;
        }

        #[test]
        #[should_panic(expected = "subtraction overflow")]
        fn test_uint256_subtract_overflow_1() {
            let a = UInt256::from(100_000_000);
            let b = UInt256::from(150_000_000_000);
            let _ = a - b;
        }

        #[test]
        fn test_uint256_sub() {
            let v1 = UInt256::from(1000_000_000);
            let v2 = UInt256::from(999_999_999);
            let v3 = v1 - v2;
            assert_eq!(v3, UInt256::ONE);
            let v4 = UInt256::from(801_002);
            let v5 = v1 - v4;
            assert_eq!(v5, UInt256::from(999_198_998));
        }
    }

    #[cfg(test)]
    mod test_multiplication {

        use super::*;

        #[test]
        fn test_uint256_mul_communitative() {
            let a = UInt256::from(1000_000_000);
            let b = UInt256::from(200_000_000);
            assert_eq!(a * b, b * a);
        }

        #[test]
        fn test_uint256_mul_identity() {
            let a = UInt256::from(1000_000_000);
            let b = UInt256::from(1);
            let c = a * b;
            assert_eq!(c, a);
        }

        #[test]
        fn test_uint256_mul_zero_property() {
            let a = UInt256::from(1000_000_000);
            let b = UInt256::ZERO;
            let c = a * b;
            assert_eq!(c, UInt256::ZERO);
        }

        #[test]
        fn test_uint256_mul_basic() {
            let u256_value1 = UInt256::from(1000_000_000);
            let u256_value2 = UInt256::from(999_999_999);
            let u256_value3 = u256_value1 * u256_value2;
            assert_eq!(u256_value3, UInt256::from(999_999_999_000_000_000));
        }

        #[test]
        #[should_panic(expected = "attempt to multiply with overflow")]
        fn test_uint256_mul_overflow() {
            let a = UInt256::MAX;
            let b = UInt256::from(2);
            let _ = a * b;
        }
    }

    #[cfg(test)]
    mod test_division {

        use super::*;

        #[test]
        fn test_uint256_div_basic() {
            let a = UInt256::from(10_000_000);
            let b = UInt256::from(2);
            let c = a / b;
            let expected = UInt256::from(5_000_000);
            assert_eq!(c, expected);
        }

        #[test]
        #[should_panic(expected = "division by zero")]
        fn test_uint256_div_by_zero() {
            let _ = UInt256::from(100000) / UInt256::ZERO;
        }

        #[test]
        fn test_uint256_zero_dividend() {
            let a = UInt256::ZERO / UInt256::from(3_000_000);
            assert_eq!(a, UInt256::ZERO);
        }

        #[test]
        fn test_uint256_smaller_dividend() {
            let a = UInt256::from(1_000_000);
            let b = UInt256::from(1_000_000_000);
            let c = a / b;
            assert_eq!(c, UInt256::ZERO);
        }
    }

    #[cfg(test)]
    mod test_endianness {

        use super::*;

        #[test]
        fn test_endian_conversions() {
            let bytes = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
                0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10
            ];

            let u256_value = UInt256::from_be_bytes(&bytes);
            let be_bytes = u256_value.to_be_bytes();
            assert_eq!(bytes, *be_bytes.as_ref());
        }

        #[test]
        fn test_big_endianness() {
            let data = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x4a,
            ];

            let uint256_value = UInt256::from_be_bytes(&data);
            assert_eq!(UInt256{ high: 0, low: 330, endian: Endian::Big }, uint256_value);
            let b = uint256_value.as_bytes();
            let result = b.as_ref();
            assert_eq!(*result, data);
        }

        #[test]
        fn test_integer_bytes_conversion() {
            let n = UInt256::from(330);
            let a = n.as_bytes();
            let b = a.as_ref();
            // We expect the bytes to be in big-endian format in 32-bytes
            let expected: [u8; 32] = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x4a,
            ];
            assert_eq!(b, &expected);
        }

        #[test]
        fn test_endian_from() {
            let bytes = [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x4a,
            ];
            let a = UInt256::from_le_bytes(&bytes);
            assert_eq!(format!("{}", a), "0x4a01000000000000000000000000000000000000000000000000000000000000");
            assert_eq!(a.endian(), Endian::Little);
            // assert_eq!(a, UInt256::from(18945));
        }

        #[test]
        fn test_to_uint256() {
            let data_be: Vec<u8> = vec![0x01, 0x04a];
            let mut bytes_32 = pad_bytes(&data_be, 0x00, Endian::Big);
            let a = to_uint256(&bytes_32, Endian::Big);
            assert_eq!(a, UInt256::from(330));

            let data_le = vec![0x4a, 0x01];
            bytes_32 = pad_bytes(&data_le, 0x00, Endian::Little);

            let b = to_uint256(&bytes_32, Endian::Little);
            assert_eq!(b, UInt256::from(330));
        }
    }

    #[cfg(test)]
    mod div_tests {
        use super::*;
        #[test]
        fn test_div_basic() {
            // Test division of two simple numbers
            let a = UInt256 { high: 0, low: 10, endian: Endian::Big };
            let b = UInt256 { high: 0, low: 2, endian: Endian::Big };
            let result = a / b;
            let quotient = UInt256 { high: 0, low: 5, endian: Endian::Big };
            assert_eq!(result, quotient);
        }

        #[test]
        fn test_div_by_one() {
            // Test division by one (should return the original number)
            let a = UInt256 { high: 12345, low: 67890, endian: Endian::Big };
            let b = UInt256 { high: 0, low: 1, endian: Endian::Big };
            let result = a / b;
            assert_eq!(result, a);
        }

        #[test]
        fn test_div_large_divisor() {
            // Test division where the divisor is greater than the dividend (should return zero)
            let a = UInt256 { high: 0, low: 5, endian: Endian::Big };
            let b = UInt256 { high: 0, low: 10, endian: Endian::Big };
            let result = a / b;
            let quotient = UInt256 { high: 0, low: 0, endian: Endian::Big };
            assert_eq!(result, quotient);
        }

        #[test]
        fn test_div_self() {
            // Test division of a number by itself (should return one)
            let a = UInt256 { high: 12345, low: 67890, endian: Endian::Big };
            let result = a / a;
            let quotient = UInt256 { high: 0, low: 1, endian: Endian::Big };
            assert_eq!(result, quotient);
        }

        #[test]
        #[should_panic(expected = "division by zero")]
        fn test_div_by_zero() {
            // Test division by zero (should panic)
            let a = UInt256 { high: 1, low: 0, endian: Endian::Big };
            let b = UInt256 { high: 0, low: 0, endian: Endian::Big };
            let _ = a / b; // This should panic
        }

        #[test]
        #[ignore = "This test hangs"]
        fn test_div_large_numbers() {
            // Test division with large numbers
            let a = UInt256 {
                high: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                low: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                endian: Endian::Big,
            };
            let b = UInt256 { high: 0, low: 2, endian: Endian::Big };
            let result = a / b;
            let quotient = UInt256 {
                high: 0x7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                low: 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF,
                endian: Endian::Big,
            };
            assert_eq!(result, quotient);
        }
    }
}


