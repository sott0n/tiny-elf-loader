use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::mem;

#[derive(Debug)]
pub struct ElfHeader<'input> {
    pub basic_info: &'input ElfBaseInfo,
    //pub info: ElfInfo,
}

#[derive(Debug)]
pub struct ElfBaseInfo {
    pub magic: [u8; 4],
    pub class: Class,
    pub data: ByteOrder,
    pub version: Version,
    pub os_abi: OsAbi,
    pub abi_version: u8,
    pub padding: [u8; 7],
}

#[derive(Debug)]
pub enum Class {
    C32 = 1,
    C64 = 2, 
}

#[derive(Debug)]
pub enum ByteOrder {
    LE = 1,
    BE = 2,
}

#[derive(Debug)]
pub enum Version {
    Current = 1,
}

#[derive(Debug)]
pub enum OsAbi {
    SystemV = 0x00,
    HpUx    = 0x01,
    NetBSD  = 0x02,
    Linux   = 0x03,
    Solaris = 0x04,
    Aix     = 0x07,
    Irix    = 0x08,
    FreeBSD = 0x09,
    OpenBSD = 0x0C,
    OpenVMS = 0x0D,
    // TODO: many more..
}

pub const MAGIC: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

pub fn parse_header<'input>(input: &'input [u8]) -> ElfHeader<'input> {
    let size_b_header = mem::size_of::<ElfBaseInfo>();
    let b_header: &'input ElfBaseInfo = read_binary(&input[..size_b_header]);
    assert!(b_header.magic == MAGIC);

    ElfHeader {basic_info: b_header}
}

pub fn read_binary<T>(input: &[u8]) -> &T {
    assert!(mem::size_of::<T>() <= input.len());
    unsafe { &*(input.as_ptr() as *const T) }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("This ELF loader's args must be only object file name.");
        exit(1);
    }

    let filename = &args[1];
    if !filename.ends_with(".o") {
        eprintln!("Arg filename must be `.o`");
        exit(1);
    }

    let mut f = File::open(filename).unwrap();
    let mut buf = Vec::new();
    assert!(f.read_to_end(&mut buf).unwrap() > 0);
    let header = parse_header(&buf);
    dbg!(header);
}
