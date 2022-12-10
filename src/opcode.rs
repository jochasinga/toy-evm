use std::{num::ParseIntError, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
pub struct Opcode(pub u8);

impl Opcode {
    pub const ADD: Opcode = Opcode(0x01);
    pub const MUL: Opcode = Opcode(0x02);
    pub const SUB: Opcode = Opcode(0x03);
    pub const ISZERO: Opcode = Opcode(0x15);
    pub const POP: Opcode = Opcode(0x50);
    pub const MSTORE: Opcode = Opcode(0x52);
    pub const SSTORE: Opcode = Opcode(0x55);
    pub const PUSH1: Opcode = Opcode(0x60);
    pub const PUSH2: Opcode = Opcode(0x61);
    pub const DUP1: Opcode = Opcode(0x80);
    pub const DUP2: Opcode = Opcode(0x81);
    pub const SWAP1: Opcode = Opcode(0x90);

    /// Read opcodes from a file generated by solc.
    pub fn from_file(filename: &str) -> Vec<Opcode> {
        let mut opcodes = Vec::new();
        let file = std::fs::read_to_string(filename).unwrap();
        for split in file.split_whitespace() {
            let opcode = split.parse::<Opcode>().unwrap();
            opcodes.push(opcode);
        }
        opcodes
    }
}

impl FromStr for Opcode {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("0x");
        match u8::from_str_radix(s, 16) {
            Ok(u) => match u {
                0x01 => Ok(Self::ADD),
                0x02 => Ok(Self::MUL),
                0x03 => Ok(Self::SUB),
                0x15 => Ok(Self::ISZERO),
                0x50 => Ok(Self::POP),
                0x52 => Ok(Self::MSTORE),
                0x55 => Ok(Self::SSTORE),
                0x60 => Ok(Self::PUSH1),
                0x80 => Ok(Self::DUP1),
                0x81 => Ok(Self::DUP2),
                0x90 => Ok(Self::SWAP1),
                v => Ok(Self(v)),
            },
            Err(e) => match s {
                "ADD" => Ok(Self::ADD),
                "MUL" => Ok(Self::MUL),
                "SUB" => Ok(Self::SUB),
                "ISZERO" => Ok(Self::ISZERO),
                "POP" => Ok(Self::POP),
                "MSTORE" => Ok(Self::MSTORE),
                "SSTORE" => Ok(Self::SSTORE),
                "PUSH1" => Ok(Self::PUSH1),
                "DUP1" => Ok(Self::DUP1),
                "DUP2" => Ok(Self::DUP2),
                "SWAP1" => Ok(Self::SWAP1),
                _ => Err(e),
            },
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_file() {
        let opcodes = Opcode::from_file("tests/Example.opcode");
        assert_eq!(opcodes.len(), 5);
        assert_eq!(opcodes[0], Opcode::PUSH1);
        assert_eq!(opcodes[1], Opcode(0x80));
        assert_eq!(opcodes[2], Opcode::PUSH1);
        assert_eq!(opcodes[3], Opcode(0x40));
        assert_eq!(opcodes[4], Opcode::ISZERO);
    }

    #[test]
    fn test_from_str() {
        let mut add = "0x01".parse::<Opcode>().unwrap();
        assert_eq!(add, Opcode::ADD);
        add = "01".parse::<Opcode>().unwrap();
        assert_eq!(add, Opcode::ADD);
        add = "ADD".parse::<Opcode>().unwrap();
        assert_eq!(add, Opcode::ADD);

        let mut mul = "0x02".parse::<Opcode>().unwrap();
        assert_eq!(mul, Opcode::MUL);
        mul = "02".parse::<Opcode>().unwrap();
        assert_eq!(mul, Opcode::MUL);
        mul = "MUL".parse::<Opcode>().unwrap();
        assert_eq!(mul, Opcode::MUL);

        let mut sub = "0x03".parse::<Opcode>().unwrap();
        assert_eq!(sub, Opcode::SUB);
        sub = "03".parse::<Opcode>().unwrap();
        assert_eq!(sub, Opcode::SUB);
        sub = "SUB".parse::<Opcode>().unwrap();
        assert_eq!(sub, Opcode::SUB);

        let mut iszero = "0x15".parse::<Opcode>().unwrap();
        assert_eq!(iszero, Opcode::ISZERO);
        iszero = "15".parse::<Opcode>().unwrap();
        assert_eq!(iszero, Opcode::ISZERO);
        iszero = "ISZERO".parse::<Opcode>().unwrap();
        assert_eq!(iszero, Opcode::ISZERO);

        let mut pop = "0x50".parse::<Opcode>().unwrap();
        assert_eq!(pop, Opcode::POP);
        pop = "50".parse::<Opcode>().unwrap();
        assert_eq!(pop, Opcode::POP);
        pop = "POP".parse::<Opcode>().unwrap();
        assert_eq!(pop, Opcode::POP);

        let mut sstore = "0x55".parse::<Opcode>().unwrap();
        assert_eq!(sstore, Opcode::SSTORE);
        sstore = "55".parse::<Opcode>().unwrap();
        assert_eq!(sstore, Opcode::SSTORE);
        sstore = "SSTORE".parse::<Opcode>().unwrap();
        assert_eq!(sstore, Opcode::SSTORE);

        let mut push1 = "0x60".parse::<Opcode>().unwrap();
        assert_eq!(push1, Opcode::PUSH1);
        push1 = "60".parse::<Opcode>().unwrap();
        assert_eq!(push1, Opcode::PUSH1);
        push1 = "PUSH1".parse::<Opcode>().unwrap();
        assert_eq!(push1, Opcode::PUSH1);

        let mut dup1 = "0x80".parse::<Opcode>().unwrap();
        assert_eq!(dup1, Opcode::DUP1);
        dup1 = "80".parse::<Opcode>().unwrap();
        assert_eq!(dup1, Opcode::DUP1);
        dup1 = "DUP1".parse::<Opcode>().unwrap();
        assert_eq!(dup1, Opcode::DUP1);

        let mut dup2 = "0x81".parse::<Opcode>().unwrap();
        assert_eq!(dup2, Opcode::DUP2);
        dup2 = "81".parse::<Opcode>().unwrap();
        assert_eq!(dup2, Opcode::DUP2);
        dup2 = "DUP2".parse::<Opcode>().unwrap();
        assert_eq!(dup2, Opcode::DUP2);

        let mut swap1 = "0x90".parse::<Opcode>().unwrap();
        assert_eq!(swap1, Opcode::SWAP1);
        swap1 = "90".parse::<Opcode>().unwrap();
        assert_eq!(swap1, Opcode::SWAP1);
        swap1 = "SWAP1".parse::<Opcode>().unwrap();
        assert_eq!(swap1, Opcode::SWAP1);
    }
}
