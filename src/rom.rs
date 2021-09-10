use std::str;

// http://problemkaputt.de/pandocs.htm#thecartridgeheader
const _ROM_START: u16 = 0x0100;
const _NINTENDO_LOGO_START: u16 = 0x0104;
const TITLE_START: u16 = 0x0134;
const MANUFACTURER_CODE_START: u16 = 0x013F;

pub struct ROM {
    pub title: String,
    pub manufacturer_code: String,
}

impl ROM {
    pub fn from_bytes(rom_bytes: &[u8]) -> Option<ROM> {
        let title = ROM::read_title(rom_bytes);
        let manufacturer_code = ROM::read_manufacturer_code(rom_bytes);

        Some(ROM {
            title,
            manufacturer_code,
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
}
