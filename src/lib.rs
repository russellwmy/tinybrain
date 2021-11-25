use std::io::Read;

const INS: &[u8] = &[
    b'>', b'<', b'+', b'-', b'.', b',', b'[', b']', b' ', b'\t', b'\n',
];

fn readchar() -> u8 {
    let mut stdin = std::io::stdin();
    let mut buf = [0; 1];
    stdin.read(&mut buf).unwrap();
    buf[0]
}

fn check_instructions(ins: &String) -> bool {
    for c in ins.as_bytes() {
        if !INS.contains(&c) {
            return false;
        }
    }
    true
}
pub fn process(ins: String) -> Vec<u8> {
    let mut ins_ptr: usize = 0;
    let mut mem_ptr: usize = 0;
    let mut mem: Vec<u8> = vec![0; 30000];
    let mut result: Vec<u8> = vec![];

    assert_eq!(check_instructions(&ins), true);

    while ins_ptr < ins.len() {
        let c = ins.as_bytes()[ins_ptr];
        match c {
            b'+' => mem[mem_ptr] += 1,
            b'-' => mem[mem_ptr] -= 1,
            b'<' => mem_ptr -= 1,
            b'>' => mem_ptr += 1,
            b'.' => result.push(mem[mem_ptr]),
            b',' => mem[mem_ptr] = readchar(),
            b'[' => {
                if mem[mem_ptr] == 0 {
                    let mut depth = 1;
                    ins_ptr += 1;
                    while depth > 0 {
                        match ins.as_bytes()[ins_ptr] {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            _ => (),
                        }
                        ins_ptr += 1;
                    }
                }
            }
            b']' => {
                if mem[mem_ptr] != 0 {
                    let mut depth = 1;
                    ins_ptr -= 1;
                    while depth > 0 {
                        match ins.as_bytes()[ins_ptr] {
                            b'[' => depth -= 1,
                            b']' => depth += 1,
                            _ => (),
                        }
                        ins_ptr -= 1;
                    }
                }
            }
            _ => (),
        }
        ins_ptr += 1;
    }

    result
}
