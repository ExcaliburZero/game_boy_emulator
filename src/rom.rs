use std::str;

// http://problemkaputt.de/pandocs.htm#thecartridgeheader
const _ROM_START: u16 = 0x0100;
const _NINTENDO_LOGO_START: u16 = 0x0104;
const TITLE_START: u16 = 0x0134;
const MANUFACTURER_CODE_START: u16 = 0x013F;
const CBG_FLAG_ADDRESS: u16 = 0x0143;

pub struct ROM {
    pub title: String,
    pub manufacturer_code: String,
    pub cbg_flag: CbgFlag,
}

impl ROM {
    pub fn from_bytes(rom_bytes: &[u8]) -> Option<ROM> {
        let title = ROM::read_title(rom_bytes);
        let manufacturer_code = ROM::read_manufacturer_code(rom_bytes);
        let cbg_flag = ROM::read_cbg_flag(rom_bytes).unwrap();

        Some(ROM {
            title,
            manufacturer_code,
            cbg_flag,
        })
    }

    fn read_title(rom_bytes: &[u8]) -> String {
        let ascii_bytes: Vec<u8> = (0..15)
            .map(|i| rom_bytes[(TITLE_START + i) as usize])
            .collect();

        str::from_utf8(&ascii_bytes).unwrap().to_string()
    }

    fn read_manufacturer_code(rom_bytes: &[u8]) -> String {
        // TODO: verify that this is reading the correct part of the ROM
        let ascii_bytes: Vec<u8> = (0..3)
            .map(|i| rom_bytes[(MANUFACTURER_CODE_START + i) as usize])
            .collect();

        str::from_utf8(&ascii_bytes).unwrap().to_string()
    }

    fn read_cbg_flag(rom_bytes: &[u8]) -> Option<CbgFlag> {
        // TODO: verify that this is reading the correct part of the ROM
        rom_bytes
            .get(CBG_FLAG_ADDRESS as usize)
            .map(|byte| CbgFlag::from_byte(*byte))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CbgFlag {
    SupportsCbg(),
    OnlyWorksOnCbg(),
    Other(u8),
}

impl CbgFlag {
    pub fn from_byte(byte: u8) -> CbgFlag {
        match byte {
            0x80 => CbgFlag::SupportsCbg(),
            0xC0 => CbgFlag::OnlyWorksOnCbg(),
            b => CbgFlag::Other(b),
        }
    }
}
