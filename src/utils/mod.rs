pub struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Reader { bytes, pos: 0 }
    }

    pub fn read_delimited_usize(&mut self, delimiter: u8) -> usize {
        let mut value: usize = 0;
        while self.bytes[self.pos] != delimiter {
            value = 10 * value + (self.bytes[self.pos] & 0x0f) as usize;
            self.pos += 1;
        }
        // skip delimiter
        self.pos += 1;

        value
    }

    pub fn has_next(&self) -> bool {
        self.pos < self.bytes.len()
    }

    pub fn skip(&mut self, n: usize) {
        self.pos += n;
    }

    pub fn skip_lit(&mut self, lit: &[u8]) {
        debug_assert_eq!(lit, &self.bytes[self.pos..self.pos + lit.len()]);
        self.skip(lit.len());
    }
}
