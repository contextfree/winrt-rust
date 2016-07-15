use std::{fmt, cmp};

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Guid(pub ::w::GUID); // TODO: field should be private (probably requires const fn)

impl Guid {
    pub fn as_iid(&self) -> ::w::IID {
        self.0
    } 
}

impl From<::w::GUID> for Guid {
    fn from(guid: ::w::GUID) -> Self {
        Guid(guid)
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.0.Data1, self.0.Data2, self.0.Data3,
            self.0.Data4[0], self.0.Data4[1], self.0.Data4[2], self.0.Data4[3],
            self.0.Data4[4], self.0.Data4[5], self.0.Data4[6], self.0.Data4[7])
    }
}

impl cmp::PartialEq<Guid> for Guid {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0.Data1 == other.0.Data1 && self.0.Data2 == other.0.Data2 && self.0.Data3 == other.0.Data3 && self.0.Data4 == other.0.Data4
    }
}

impl cmp::Eq for Guid {}