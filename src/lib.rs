//! Readers for `&str`s and `String`s instead of `u8`s.
//!
//! See [`RealStrRead`] and [`StringRead`] as the traits, and [`StrReader`] and [`StringReader`] as
//! the structs.
use std::collections::VecDeque;

/// The base trait that both `RealStrRead` and `StringRead` need to implement.
pub trait StrRead {
    /// Get a reference to the next `&str`.
    ///
    /// Returns `None` if it's empty.
    fn peek_str(&self) -> Option<&str>;
    // fn peek_mut_str<'a>(&'a mut self) -> Option<&'a mut str>;

    // fn map_str(&mut self, mut f: impl FnMut(&mut str)) {
    //     if let Some(s) = self.peek_mut_str() {
    //         f(s)
    //     }
    // }

    // fn map_str(&mut self, f: impl FnMut(&mut str));

    /// Check if there is nothing to pop.
    fn is_empty(&self) -> bool {
        self.peek_str().is_none()
    }
}

/// Represent anything that pops out `&str`.
pub trait RealStrRead: StrRead {
    /// Remove the next `&str` and return it.
    ///
    /// Returns `None` if it's empty.
    fn pop_str(&mut self) -> Option<&str>;
}

/// Represent anything that pops out `String`.
pub trait StringRead: StrRead {
    /// Remove the next `String` and return it.
    fn pop_string(&mut self) -> Option<String>;
    /// Get a mutable reference to the next `String`.
    fn peek_mut_string(&mut self) -> Option<&mut String>;

    /// Change the next `String` that will be poped.
    fn map_string(&mut self, f: impl FnMut(&mut String)) {
        self.peek_mut_string().map(f);
    }
}

/// Write/insert operations with `&str`-type readers.
pub trait StrWrite<'a> {
    /// Insert a `&str` into the reader.
    ///
    /// The newly inserted `&str` will be the *last* item in the list.
    ///
    /// # Examples
    /// ```rust
    /// let sread = StrReader::default();
    /// sread.push_str("hai");
    /// sread.push_str("bai");
    /// assert_eq!(sread.pop_str(), Some("hai"));
    /// assert_eq!(sread.pop_str(), Some("bai"));
    /// assert_eq!(sread.pop_str(), None);
    /// ```
    fn push_str(&'a mut self, s: &'a str);

    /// Insert a `&str` into the reader.
    ///
    /// The newly inserted `&str` will be the *next* item to be returned.
    ///
    /// # Examples
    /// ```rust
    /// let sread = StrReader::default();
    /// sread.shift_str("hai");
    /// sread.shift_str("bai");
    /// assert_eq!(sread.pop_str(), Some("bai"));
    /// assert_eq!(sread.pop_str(), Some("hai"));
    /// assert_eq!(sread.pop_str(), None);
    /// ```
    fn shift_str(&'a mut self, s: &'a str);
}

/// Write/insert operations with `String`-type readers.
pub trait StringWrite {
    /// Insert a `String` into the reader.
    ///
    /// The newly inserted `String` will be the *last* item in the list.
    ///
    /// # Examples
    /// ```rust
    /// let sread = StringReader::default();
    /// sread.push_string("hai".to_string());
    /// sread.push_string("bai".to_string());
    /// assert_eq!(sread.pop_string(), Some("hai".to_string()));
    /// assert_eq!(sread.pop_string(), Some("bai".to_string()));
    /// assert_eq!(sread.pop_string(), None);
    /// ```
    fn push_string(&mut self, s: String);
    /// Insert a `String` into the reader.
    ///
    /// The newly inserted `String` will be the *last* item in the list.
    ///
    /// # Examples
    /// ```rust
    /// let sread = StringReader::default();
    /// sread.shift_string("hai".to_string());
    /// sread.shift_string"bai".to_string());
    /// assert_eq!(sread.pop_string(), Some("bai".to_string()));
    /// assert_eq!(sread.pop_string(), Some("hai".to_string()));
    /// assert_eq!(sread.pop_string(), None);
    /// ```
    fn shift_string(&mut self, s: String);
}

impl StrRead for String {
    fn peek_str(&self) -> Option<&str> {
        Some(self)
    }

    // fn map_str(&mut self, mut f: impl FnMut(&mut str)) {
    //     f(self)
    // }

    // fn peek_mut_str<'a>(&'a mut self) -> Option<&'a mut str> {
    //     Some(self)
    // }
}
impl StringRead for String {
    fn pop_string(&mut self) -> Option<String> {
        Some(std::mem::take(self))
    }

    fn map_string(&mut self, mut f: impl FnMut(&mut String)) {
        f(self);
    }

    fn peek_mut_string(&mut self) -> Option<&mut String> {
        Some(self)
    }
}

// NOTE: #[derive(Default)] is not possible, it requires R to impl Default

/// An equivalent of `std::io::BufReader` but for `String` instead of `char`.
#[derive(Clone, Debug)]
pub struct StringReader<R: StringRead = String> {
    pub queue: VecDeque<String>,
    pub reader: Option<R>,
}

impl<R: StringRead> Default for StringReader<R> {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            reader: None,
        }
    }
}

impl<R: StringRead> From<R> for StringReader<R> {
    fn from(value: R) -> Self {
        Self {
            queue: Default::default(),
            reader: Some(value),
        }
    }
}

impl<R: StringRead> From<VecDeque<String>> for StringReader<R> {
    fn from(value: VecDeque<String>) -> Self {
        Self {
            queue: value,
            reader: None,
        }
    }
}

impl<R: StringRead> StringReader<R> {
    /// Equivalent to `default()`.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<R: StringRead> StrRead for StringReader<R> {
    fn peek_str(&self) -> Option<&str> {
        (self.queue.front().map(|s| s.as_str()))
            .or_else(|| self.reader.as_ref().map(|r| r.peek_str())?)
    }

    // fn peek_mut_str<'a>(&'a mut self) -> Option<&'a mut str> {
    //     (self.stack.last_mut().map(|s| s.as_mut_str()))
    //         .or_else(|| self.reader.as_mut().map(|r| r.peek_mut_str())?)
    // }

    fn is_empty(&self) -> bool {
        self.queue.is_empty() && self.reader.as_ref().map_or(true, |r| r.is_empty())
    }
}

impl<R: StringRead> StringRead for StringReader<R> {
    fn pop_string(&mut self) -> Option<String> {
        (self.queue.pop_front()).or_else(|| self.reader.as_mut().map(|r| r.pop_string())?)
    }

    fn peek_mut_string(&mut self) -> Option<&mut String> {
        (self.queue.front_mut()).or_else(|| self.reader.as_mut().map(|r| r.peek_mut_string())?)
    }
}

impl<R: StringRead> std::io::Read for StringReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut l = buf.len();
        let mut pos = 0;
        while let Some(s) = self.peek_mut_string() {
            let slen = s.len();
            if slen > l {
                buf[pos..].copy_from_slice(s[..l].as_bytes());
                *s = s[l..].to_string();
                return Ok(buf.len());
            }
            // slen <= l
            buf[pos..pos + slen].copy_from_slice(self.pop_string().unwrap().as_bytes());
            pos += slen;
            l -= slen;
        }
        Ok(pos)
    }
}

impl<R: StringRead> std::io::BufRead for StringReader<R> {
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        if let Some(s) = self.peek_str() {
            Ok(s.as_bytes())
        } else if let Some(s) = self.reader.as_ref().and_then(|r| r.peek_str()) {
            Ok(s.as_bytes())
        } else {
            Ok(&[])
        }
    }

    fn consume(&mut self, amt: usize) {
        use std::io::Read;
        let mut buf: Vec<u8> = Vec::new();
        (0..amt).for_each(|_| buf.push(0));
        self.read(&mut buf).unwrap();
    }
}

impl StrRead for str {
    fn peek_str(&self) -> Option<&str> {
        Some(self)
    }

    // fn peek_mut_str<'a>(&'a mut self) -> Option<&'a mut str> {
    //     Some(self)
    // }
}
impl RealStrRead for str {
    fn pop_str(&mut self) -> Option<&str> {
        Some(self)
    }
}
impl<R: StrRead + ?Sized> StrRead for Box<R> {
    fn peek_str(&self) -> Option<&str> {
        (**self).peek_str()
    }

    // fn peek_mut_str<'a>(&'a mut self) -> Option<&'a mut str> {
    //     (**self).peek_mut_str()
    // }
}
impl<R: RealStrRead + ?Sized> RealStrRead for Box<R> {
    fn pop_str(&mut self) -> Option<&str> {
        (**self).pop_str()
    }
}

#[derive(Clone, Debug)]
pub struct StrReader<'a, R: RealStrRead = Box<str>> {
    pub queue: VecDeque<&'a str>,
    pub reader: Option<R>,
}

impl<'a, R: RealStrRead> Default for StrReader<'a, R> {
    fn default() -> Self {
        Self {
            queue: Default::default(),
            reader: None,
        }
    }
}

impl<'a, R: RealStrRead> From<R> for StrReader<'a, R> {
    fn from(value: R) -> Self {
        Self {
            queue: Default::default(),
            reader: Some(value),
        }
    }
}

impl<'a, R: RealStrRead> From<VecDeque<&'a str>> for StrReader<'a, R> {
    fn from(value: VecDeque<&'a str>) -> Self {
        Self {
            queue: value,
            reader: None,
        }
    }
}

impl<'a, R: RealStrRead> StrReader<'a, R> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a, R: RealStrRead> StrRead for StrReader<'a, R> {
    fn peek_str(&self) -> Option<&str> {
        (self.queue.front().copied()).or_else(|| self.reader.as_ref().and_then(|r| r.peek_str()))
    }

    // fn peek_mut_str<'b>(&'b mut self) -> Option<&'b mut str> {
    //     (self.stack.last_mut().map(|s| s.as_mut()))
    //         .or_else(|| self.reader.as_mut().map(|r| r.peek_mut_str())?)
    // }

    fn is_empty(&self) -> bool {
        self.queue.is_empty() && self.reader.as_ref().map_or(true, |r| r.is_empty())
    }
}

impl<'a, R: RealStrRead> RealStrRead for StrReader<'a, R> {
    fn pop_str(&mut self) -> Option<&str> {
        self.queue
            .pop_front()
            .or_else(|| self.reader.as_mut().and_then(|r| r.pop_str()))
    }
}

impl<'r, R: RealStrRead> StrWrite<'r> for StrReader<'r, R> {
    fn push_str(&'r mut self, s: &'r str) {
        self.queue.push_back(s);
    }

    fn shift_str(&'r mut self, s: &'r str) {
        self.queue.push_front(s);
    }
}

// impl<'r, R: RealStrRead> std::io::Read for StrReader<'r, R> {
//     fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
//         let mut l = buf.len();
//         let mut pos = 0;
//         while let Some(s) = self.pop_str() {
//             let slen = s.len();
//             if slen > l {
//                 buf[pos..].copy_from_slice(s[..l].as_bytes());
//                 self.shift_str(&s[l..]);
//                 return Ok(buf.len());
//             }
//             // slen <= l
//             buf[pos..pos + slen].copy_from_slice(s.as_bytes());
//             pos += slen;
//             l -= slen;
//         }
//         Ok(pos)
//     }
// }

impl<R: StringRead> StringWrite for StringReader<R> {
    fn push_string(&mut self, s: String) {
        self.queue.push_back(s);
    }

    fn shift_string(&mut self, s: String) {
        self.queue.push_front(s);
    }
}
