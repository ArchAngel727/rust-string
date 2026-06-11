use std::{
    alloc::{Layout, alloc, dealloc, realloc},
    fmt::Display,
    ops::Deref,
    ptr::{self, NonNull, copy_nonoverlapping},
};

pub struct String {
    ptr: NonNull<u8>,
    len: usize,
    capacity: usize,
}

impl String {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    /// # Panics
    ///
    /// Will panic if layout fails to be built
    #[must_use]
    pub fn new_with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self::new();
        }

        let layout = Layout::array::<u8>(capacity).expect("Invalid capacity");
        let raw_ptr = unsafe { alloc(layout) };
        let ptr = NonNull::new(raw_ptr).unwrap_or_else(|| std::alloc::handle_alloc_error(layout));

        Self {
            ptr,
            len: 0,
            capacity,
        }
    }

    fn grow_to(&mut self, new_capacity: usize) {
        let new_layout = Layout::array::<u8>(new_capacity).expect("Invalid capacity");

        let new_raw_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<u8>(self.capacity).expect("Invalid capacity");
            unsafe { realloc(self.ptr.as_ptr(), old_layout, new_capacity) }
        };

        self.ptr =
            NonNull::new(new_raw_ptr).unwrap_or_else(|| std::alloc::handle_alloc_error(new_layout));
        self.capacity = new_capacity;
    }

    fn reserve(&mut self, ammount: usize) {
        let needed = self.len.checked_add(ammount).expect("Capacity overflow");

        if self.capacity >= needed {
            return;
        }

        let new_capacity = needed.max(self.capacity * 2).max(8);
        self.grow_to(new_capacity);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub const fn clear(&mut self) {
        if self.capacity == 0 {
            return;
        }

        unsafe {
            ptr::write_bytes(self.ptr.as_ptr(), 0, self.len);
        }

        self.len = 0;
    }

    /// # Panics
    ///
    /// Will panic if layout fails to be built
    pub fn erase(&mut self) {
        if self.capacity != 0 {
            let layout = Layout::array::<u8>(self.capacity).expect("Invalid capacity");
            unsafe {
                dealloc(self.ptr.as_ptr(), layout);
            }
        }

        self.ptr = NonNull::dangling();
        self.len = 0;
        self.capacity = 0;
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    pub const fn push(&mut self, byte: u8) {
        unsafe {
            self.ptr.as_ptr().add(self.len).write(byte);
        }

        self.len += 1;
    }

    pub fn push_char(&mut self, chr: char) {
        let mut bytes = [0u8; 4];
        let utf_8_encoded = chr.encode_utf8(&mut bytes);
        self.push_str(utf_8_encoded);
    }

    pub fn push_str(&mut self, str: &str) {
        let bytes = str.as_bytes();
        self.reserve(bytes.len());

        unsafe {
            copy_nonoverlapping(bytes.as_ptr(), self.ptr.as_ptr().add(self.len), bytes.len());
        }

        self.len += bytes.len();
    }

    /// # Panics
    ///
    /// Will panic if layout fails to be built
    pub fn shrink_to_fit(&mut self) {
        if self.len == self.capacity {
            return;
        }

        let new_layout = Layout::array::<u8>(self.len).expect("Invalid capacity");

        let new_raw_ptr = if self.capacity == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<u8>(self.capacity).expect("Invalid capacity");
            unsafe { realloc(self.ptr.as_ptr(), old_layout, self.len) }
        };

        self.ptr =
            NonNull::new(new_raw_ptr).unwrap_or_else(|| std::alloc::handle_alloc_error(new_layout));
        self.capacity = self.len;
    }
}

impl Default for String {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for String {
    fn drop(&mut self) {
        if self.capacity == 0 {
            return;
        }

        let layout = Layout::array::<u8>(self.capacity).expect("Invalid capacity");
        unsafe {
            dealloc(self.ptr.as_ptr(), layout);
        }
    }
}

impl Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let mut str = Self::new_with_capacity(value.len());
        str.push_str(value);

        str
    }
}

impl Clone for String {
    fn clone(&self) -> Self {
        let mut clone = Self::new_with_capacity(self.capacity);
        clone.push_str(self.as_str());

        clone
    }
}

impl PartialEq for String {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<&str> for String {
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for String {}

impl Deref for String {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
