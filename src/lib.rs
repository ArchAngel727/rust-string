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
    /// Creates a new `String` without allocating
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            capacity: 0,
        }
    }

    /// Creates a new `String` with a given capacity
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is large enough that the resulting allocation
    /// size would overflow `isize`.
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

    /// Returns the `len` of the `String`
    #[must_use]
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns a `bool` showing if the `String` is empty
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the `capacity` of the `Sting`
    #[must_use]
    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    /// Clears a `String`'s buffer and sets the length to 0. Does not touch the memory.
    pub const fn clear(&mut self) {
        if self.capacity == 0 {
            return;
        }

        unsafe {
            ptr::write_bytes(self.ptr.as_ptr(), 0, self.len);
        }

        self.len = 0;
    }

    /// Deallocates this string's buffer and resets it to an empty, unallocated state.
    ///
    /// After this call, the string has zero length and zero capacity, equivalent
    /// to a freshly constructed `MyString::new()`. The next push will trigger a
    /// new allocation.
    /// # Panics
    ///
    /// In theory, panics if the layout for the current capacity cannot be built.
    /// In practice this cannot occur, since the buffer was originally allocated
    /// with this same layout.
    pub fn erase(&mut self) {
        if self.capacity != 0 {
            let layout = Layout::array::<u8>(self.capacity).expect("Invalid capacity");
            // SAFETY: self.ptr was allocated with this exact layout (matching
            // self.capacity), and is freed at most once because we reset capacity
            // to 0 immediately after, which makes Drop skip a second dealloc.
            unsafe {
                dealloc(self.ptr.as_ptr(), layout);
            }
        }

        self.ptr = NonNull::dangling();
        self.len = 0;
        self.capacity = 0;
    }

    /// Returns the contents of this string as a byte slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        // SAFETY: bytes 0..self.len are initialized and not mutated through &self.
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }

    /// Returns the contents of this string as a string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str {
        // SAFETY: contents are valid UTF-8 because the only ways to append bytes
        // are push_str(&str) and push(char), both of which preserve UTF-8 validity.
        unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
    }

    /// Pushes an UTF-8 char at the end of the `String`. If the buffer is full
    /// allocates another `capacity` on the heap
    pub fn push_char(&mut self, chr: char) {
        let mut bytes = [0u8; 4];
        let utf_8_encoded = chr.encode_utf8(&mut bytes);
        self.push_str(utf_8_encoded);
    }

    /// Pushes an UTF-8 string at the end of the `String`. If the buffer is full
    /// allocates another `capacity` on the heap
    pub fn push_str(&mut self, str: &str) {
        if str.is_empty() {
            return;
        }

        let bytes = str.as_bytes();
        self.reserve(bytes.len());

        // SAFETY:
        // dst has room for `bytes.len()` more writes (just reserved); `src` and
        // `dst` are in different allocations so they cannot overlap.
        unsafe {
            copy_nonoverlapping(bytes.as_ptr(), self.ptr.as_ptr().add(self.len), bytes.len());
        }

        self.len += bytes.len();
    }

    /// Shrinks the buffer's capacity to match the current length.
    ///
    /// If the string is empty, this deallocates the buffer entirely, returning
    /// the string to its unallocated state.
    pub fn shrink_to_fit(&mut self) {
        if self.len == self.capacity {
            return;
        }

        // Shrinking to zero means deallocating — realloc to zero size is UB.
        if self.len == 0 {
            let old_layout =
                Layout::array::<u8>(self.capacity).expect("layout for current capacity");
            // SAFETY: self.ptr was allocated with old_layout. We reset capacity to 0
            // immediately, so Drop will skip a second dealloc.
            unsafe {
                dealloc(self.ptr.as_ptr(), old_layout);
            }
            self.ptr = NonNull::dangling();
            self.capacity = 0;
            return;
        }

        // At this point: self.capacity > self.len > 0.
        let old_layout = Layout::array::<u8>(self.capacity).expect("layout for current capacity");
        let new_layout = Layout::array::<u8>(self.len).expect("layout for new (smaller) capacity");

        // SAFETY: self.ptr was allocated with old_layout. self.len > 0, so the
        // new size is non-zero. self.len <= self.capacity so the realloc shrinks.
        let new_raw_ptr = unsafe { realloc(self.ptr.as_ptr(), old_layout, self.len) };

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
