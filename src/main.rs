use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::mem;

pub type P32 = u32;
pub type P64 = u64;

#[derive(Debug)]
pub struct ElfHeader<'input> {
    pub basic_info: &'input ElfBaseInfo,
    pub info: ElfInfo<'input>,
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

#[derive(Debug)]
pub enum ElfInfo<'input> {
    H32(&'input ElfInfo_<P32>),
    H64(&'input ElfInfo_<P64>),
}

#[derive(Debug)]
pub struct ElfInfo_<P> {
    pub type_: Type_,
    pub machine: Machine,
    pub version: u32,
    pub entry_point: P,
    pub ph_offset: P,
    pub sh_offset: P,
    pub flags: u32,
    pub header_size: u16,
    pub ph_entry_size: u16,
    pub ph_count: u16,
    pub sh_entry_size: u16,
    pub sh_count: u16,
    pub sh_str_index: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Type_(u16);

impl Type_ {
    fn as_type(self) -> Type {
        match self.0 {
            0 => Type::None,
            1 => Type::Relocatable,
            2 => Type::Executable,
            3 => Type::SharedObject,
            4 => Type::Core,
            x => Type::ProcessorSpecific(x),
        }
    }
}

#[derive(Debug)]
pub enum Type {
    None,
    Relocatable,
    Executable,
    SharedObject,
    Core,
    ProcessorSpecific(u16),
}

#[derive(Debug)]
pub enum Machine {
    None    = 0,
    Sparc   = 0x02,
    X86     = 0x03,
    Mips    = 0x08,
    PowerPC = 0x14,
    Arm     = 0x28,
    SuperH  = 0x2A,
    Ia64    = 0x32,
    X86_64  = 0x3E,
    AArch64 = 0xB7,
    // TODO: many more..
}

pub fn parse_header<'input>(input: &'input [u8]) -> ElfHeader<'input> {
    let size_b_header = mem::size_of::<ElfBaseInfo>();
    let basic_info: &'input ElfBaseInfo = read_binary(&input[..size_b_header]);
    assert!(basic_info.magic == MAGIC);

    let info = match basic_info.class {
        Class::C32 => {
            let info: &'input ElfInfo_<P32> = read_binary(&input[..size_b_header+mem::size_of::<ElfInfo_<P32>>()]);
            ElfInfo::H32(info)
        }
        Class::C64 => {
            let info: &'input ElfInfo_<P64> = read_binary(&input[..size_b_header+mem::size_of::<ElfInfo_<P64>>()]);
            ElfInfo::H64(info)
        }
    };

    ElfHeader {
        basic_info,
        info,
    }
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
