use core::fmt::{self, Write};

struct Writer {
    buffer_point: *mut u8,
    col_pos: usize,
    color_code: u8
}

impl Writer {
    pub fn write_byte(&mut self,byte: u8) {
        let vga_offset = self.col_pos * 2;
        unsafe {
            *self.buffer_point.add(vga_offset) = byte;
            *self.buffer_point.add(vga_offset + 1) = self.color_code;
        }
        self.col_pos += 1;
    }

     pub fn write_string(&mut self,text: &str) {
           for byte in text.bytes() {
                self.write_byte(byte);
           }
    }
}

pub fn write_string(text: &str) {
    let mut writer = Writer {
        buffer_point: 0xb8000 as *mut u8,
        col_pos: 0,
        color_code: 0x0f,
    };
    writer.write_string(text);
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
