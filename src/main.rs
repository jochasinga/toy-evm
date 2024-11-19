use tinyevm::{
    eval_opcode, lex_bytecode,
    types::{utils::{pad_bytes, to_uint256}, Endian},
    Stack,
};

fn main() {
    let opcodes = lex_bytecode("0x6001600101").unwrap();
    let (mut stack, _storage, _memory) = eval_opcode(opcodes);
    let (last, rest) = stack.pop();
    let data = pad_bytes(&[0x02], 0x00, Endian::Big);
    assert_eq!(last.unwrap(), to_uint256(&data, Endian::Little));
    assert_eq!(*rest, Stack::EMPTY);
}
