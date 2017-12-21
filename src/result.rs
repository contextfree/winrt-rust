/// Re-export from WinAPI crate
pub type HRESULT = ::w::um::winnt::HRESULT;

// TODO: add more codes from https://msdn.microsoft.com/en-us/library/windows/desktop/dd542643(v=vs.85).aspx, especially the `RO_`-prefixed

/// Represents an error as a result of a Windows Runtime method call.
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OperationAborted,
    AccessDenied, // UnauthorizedAccessException in .NET (https://msdn.microsoft.com/en-us/library/awy7adbx(v=vs.110).aspx)
    UnspecifiedFailure,
    InvalidHandle,
    InvalidArgument,
    NoSuchInterface,
    NotImplemented, // NotImplementedException in .NET (https://msdn.microsoft.com/en-us/library/9ztbc5s1(v=vs.110).aspx)
    OutOfMemory, // OutOfMemoryException in .NET (https://msdn.microsoft.com/en-us/library/9ztbc5s1(v=vs.110).aspx)
    InvalidPointer,
    UnexpectedFailure,
    OutOfBounds,
    IllegalMethodCall,
    Other(HRESULT)
}

impl Error {
    #[inline]
    pub fn from_hresult(hr: HRESULT) -> Error {
        use Error::*;
        use ::w::shared::winerror::*;

        match hr {
            E_ABORT => OperationAborted,
            E_ACCESSDENIED => AccessDenied,
            E_FAIL => UnspecifiedFailure,
            E_HANDLE => InvalidHandle,
            E_INVALIDARG => InvalidArgument,
            E_NOINTERFACE => NoSuchInterface,
            E_NOTIMPL => NotImplemented,
            E_OUTOFMEMORY => OutOfMemory,
            E_POINTER => InvalidPointer,
            E_UNEXPECTED => UnexpectedFailure,
            E_BOUNDS => OutOfBounds,
            E_ILLEGAL_METHOD_CALL => IllegalMethodCall,
            _ => Other(hr)
        }
    }

    #[inline]
    pub fn as_hresult(&self) -> HRESULT {
        use Error::*;
        use ::w::shared::winerror::*;

        match *self { 
            OperationAborted => E_ABORT,
            AccessDenied => E_ACCESSDENIED,
            UnspecifiedFailure => E_FAIL,
            InvalidHandle => E_HANDLE,
            InvalidArgument => E_INVALIDARG,
            NoSuchInterface => E_NOINTERFACE,
            NotImplemented => E_NOTIMPL,
            OutOfMemory => E_OUTOFMEMORY,
            InvalidPointer => E_POINTER,
            UnexpectedFailure => E_UNEXPECTED,
            OutOfBounds => E_BOUNDS,
            IllegalMethodCall => E_ILLEGAL_METHOD_CALL,
            Other(hr) => hr,
        }
    }
}

/// A specialized `Result` type for Windows Runtime method calls.
pub type Result<T> = ::std::result::Result<T, Error>;