pub struct Reader<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        Reader { bytes, pos: 0 }
    }

    pub fn read_usize(&mut self) -> usize {
        let mut value: usize = 0;
        while self.peek() >= b'0' && self.peek() <= b'9' {
            value = 10 * value + (self.bytes[self.pos] & 0x0f) as usize;
            self.pos += 1;
        }

        value
    }

    pub fn read_isize(&mut self) -> isize {
        let mut value: isize = 0;
        let sign = if self.peek() == b'-' {
            self.pos += 1;
            -1
        } else {
            1
        };

        loop {
            match self.peek() {
                b @ b'0'..=b'9' => {
                    value = 10 * value + (b & 0x0f) as isize;
                    self.pos += 1;
                }
                _ => return sign * value,
            }
        }
    }

    // this function is slightly faster than the above, because it only does
    // one check per iteration instead of two.
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
        #[cfg(debug_assertions)]
        {
            let read_bytes = &self.bytes[self.pos..self.pos + lit.len()];
            if read_bytes != lit {
                let lit_str = std::str::from_utf8(lit).unwrap();
                let found_str = std::str::from_utf8(read_bytes).unwrap();
                panic!("expected {:?} but found {:?}", lit_str, found_str);
            }
        }
        self.skip(lit.len());
    }

    pub fn peek(&self) -> u8 {
        self.bytes[self.pos]
    }

    pub fn skip_while<P>(&mut self, mut predicate: P)
    where
        P: FnMut(u8) -> bool,
    {
        while predicate(self.peek()) {
            self.pos += 1;
        }
    }

    pub fn take_while<P>(&mut self, predicate: P) -> &'a [u8]
    where
        P: FnMut(u8) -> bool,
    {
        let start = self.pos;
        self.skip_while(predicate);
        &self.bytes[start..self.pos]
    }
}
