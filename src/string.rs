use std::fmt::{Display, write};

pub struct String {
    buffer: Box<[char]>,
    buffer_size: usize,
}

impl String {
    pub fn new() -> Self {
        Self {
            buffer: Box::new(['\0'; 16]),
            buffer_size: 16,
        }
    }

    fn alloc_more(&mut self) {
        let mut i = 0;
        let mut new_buf: Box<[char]> = (0..self.buffer_size * 2).map(|_| '\0').collect();

        while self.buffer[i] != '\0' || i == self.buffer_size {
            new_buf[i] = self.buffer[i];
            i += 1;
        }

        self.buffer = new_buf;
    }

    fn char_arr(&self) -> &Box<[char]> {
        &self.buffer
    }

    pub fn len(&self) -> usize {
        let mut i = 0usize;

        while self.buffer[i] != '\0' {
            i += 1;

            if i >= self.buffer_size {
                break;
            }
        }

        i
    }

    pub fn push(&mut self, chr: char) {
        let mut i = 0usize;

        while self.buffer[i] != '\0' {
            if i >= self.buffer_size {
                self.alloc_more();
            }

            i += 1;
        }

        self.buffer[i] = chr;
    }

    pub fn push_str(&mut self, str: &str) {
        let i = 0usize;

        while self.buffer[i] != '\0' {
            if i >= self.buffer_size {
                self.alloc_more();
            }
        }

        if i + str.len() > self.buffer_size {
            self.alloc_more();
        }

        for j in 0..str.len() {
            if let Some(chr) = str.chars().nth(j) {
                self.buffer[j] = chr;
            }
        }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for chr in self.char_arr() {
            write!(f, "{}", chr)?;
        }

        Ok(())
    }
}
