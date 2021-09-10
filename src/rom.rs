use std::str;

// http://problemkaputt.de/pandocs.htm#thecartridgeheader
const _ROM_START: u16 = 0x0100;
const _NINTENDO_LOGO_START: u16 = 0x0104;
const TITLE_START: u16 = 0x0134;
const _MANUFACTURER_CODE_START: u16 = 0x013F;

pub struct ROM {
    pub title: String,
}

impl ROM {
    pub fn from_bytes(rom_bytes: &[u8]) -> Option<ROM> {
        let title = ROM::read_title(rom_bytes);

        Some(ROM { title })
    }

    fn read_title(rom_bytes: &[u8]) -> String {
        let ascii_bytes: Vec<u8> = (0..15)
            .map(|i| rom_bytes[(TITLE_START + i) as usize])
            .collect();

        str::from_utf8(&ascii_bytes).unwrap().to_string()
    }
}
