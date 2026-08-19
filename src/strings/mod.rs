pub mod frag;

pub enum SmartString<'a> {
    Bytes(&'a [u8]),
    Str(&'a str),
}

impl<'a> From<&'a [u8]> for SmartString<'a> {
    fn from(b: &'a [u8]) -> Self {
        SmartString::Bytes(b)
    }
}

impl<'a, const N: usize> From<&'a [u8; N]> for SmartString<'a> {
    fn from(b: &'a [u8; N]) -> Self {
        SmartString::Bytes(b.as_slice())
    }
}

impl<'a> From<&'a str> for SmartString<'a> {
    fn from(s: &'a str) -> Self {
        SmartString::Str(s)
    }
}

impl<'a> From<&'a String> for SmartString<'a> {
    fn from(s: &'a String) -> Self {
        SmartString::Str(s.as_str())
    }
}

impl<'a> From<&'a Vec<u8>> for SmartString<'a> {
    fn from(v: &'a Vec<u8>) -> Self {
        SmartString::Bytes(v.as_slice())
    }
}
