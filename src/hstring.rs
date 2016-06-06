use ::std::ptr;
use ::std::fmt;
use ::std::cmp;

use ::winapi::*;
use ::runtimeobject::*;

pub struct HString(HSTRING);

impl<'a> From<&'a str> for HString {
    
    fn from(s: &'a str) -> Self {
        // Every UTF-8 byte results in either 1 or 2 UTF-16 bytes and we need one
        // more for the null terminator. This size expectation is correct in most cases,
        // so the vector doesn't need to reallocate.
        let mut s16: Vec<u16> = Vec::with_capacity(s.len() + 1);
        for c in s.encode_utf16() {
            s16.push(c);
        }
        let len = s16.len();
        s16.push(0x0u16); // add null-terminator
        let mut hstr = HString(ptr::null_mut());
        let slice: &[u16] = &s16;
        let res = unsafe { WindowsCreateString(slice as *const _ as LPCWSTR, len as UINT32, &mut hstr.0) };
        assert!(res == S_OK);
        hstr
    }
}

impl HString {    
    #[inline(always)]
    pub fn get(&self) -> HSTRING {
        self.0
    }
    
    #[inline(always)]
    // TODO: maybe remove this
    pub unsafe fn wrap(hstr: HSTRING) -> HString {
        HString(hstr)
    }
    
    #[inline(always)]
    pub fn empty() -> HString {
        HString(ptr::null_mut())
    }
    
    #[inline(always)]
    pub fn get_address(&mut self) -> &mut HSTRING {
        &mut self.0
    }
    
    #[inline(always)]
    pub fn len(&self) -> u32 {
        // This is okay even if pointer is null (returns 0)
        unsafe { WindowsGetStringLen(self.0) }
    }
    
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        unsafe { WindowsIsStringEmpty(self.0) != 0 }
    }
    
    #[inline(always)]
    fn internal_to_string(&self) -> String {
        unsafe {
            let mut len = 0;
            let buf = WindowsGetStringRawBuffer(self.0, &mut len);
            let slice: &[u16] = ::std::slice::from_raw_parts(buf, len as usize);
            String::from_utf16_lossy(slice)
        }
    }
    
    fn internal_cmp(&self, other: &HString) -> cmp::Ordering {
        let mut result = 0;
        assert!(unsafe { WindowsCompareStringOrdinal(self.0, other.0, &mut result) } == S_OK);
        match result {
            -1 => cmp::Ordering::Less,
            0 => cmp::Ordering::Equal,
            1 => cmp::Ordering::Greater,
            _ => unreachable!()
        }
    }
}

#[cfg(feature = "nightly")]
impl ToString for HString {
    fn to_string(&self) -> String {
        self.internal_to_string()
    }
}

impl cmp::PartialOrd for HString {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.internal_cmp(other))
    }
}

impl cmp::Ord for HString {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.internal_cmp(other)
    }
}

impl cmp::PartialEq<HString> for HString {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.internal_cmp(other) == cmp::Ordering::Equal
    }
}

impl cmp::Eq for HString {}

impl ::std::clone::Clone for HString {
    fn clone(&self) -> Self {
        let mut clone = HString::empty();
        let hres = unsafe { WindowsDuplicateString(self.0, clone.get_address()) };
        assert!(hres == S_OK);
        clone
    }
}

impl fmt::Display for HString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        formatter.write_str(&self.internal_to_string())
    }
}

impl Drop for HString {
    #[inline(always)]
    fn drop(&mut self) {
        // This is okay even if the pointer is null
        unsafe { WindowsDeleteString(self.0) };
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    
    use super::*;
    use self::test::Bencher;

    #[test]
    fn roundtrip() {
        let s = "12345";
        let hstr: HString = s.into();
        assert!(hstr.len() as usize == s.len());
        assert!(s == hstr.to_string());
    }
    
    #[test]
    fn empty() {
        let hstr = HString::empty();
        assert!(hstr.len() == 0);
        assert!(hstr.to_string().len() == 0);
    }
    
    #[test]
    fn is_empty() {
        let hstr = HString::empty();
        assert!(hstr.is_empty());
        let hstr: HString = "".into();
        assert!(hstr.is_empty());
        let hstr: HString = "\0".into();
        assert!(!hstr.is_empty());
    }
    
    #[test]
    fn clone() {
        let s = "123456789";
        let hstr: HString = s.into();
        let clone = hstr.clone();
        assert!(clone.to_string() == s);
        drop(clone);
        assert!(hstr.to_string() == s);
    }
    
    #[test]
    fn cmp() {
        let s1: HString = "AAA".into();
        let s2: HString = "BBB".into();
        let s3: HString = "AAA".into();
        
        assert!(s2 > s1);
        assert!(s2 != s3);
        assert!(s1 == s3);
    }

    #[bench]
    fn bench_create(b: &mut Bencher) {
        let s = "123456789";
        b.iter(|| {
            let _: HString = s.into();
        });;
    }
    
    #[bench]
    fn bench_to_string(b: &mut Bencher) {
        let hstr: HString = "123456789".into();
        b.iter(|| {
            let _ = hstr.to_string();
        });
    }
}