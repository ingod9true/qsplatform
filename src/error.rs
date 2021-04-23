use std::{io, fmt};
use std::sync::atomic::{Ordering, AtomicBool};

pub struct Error {
    handled: AtomicBool,
    kind: ErrorKind
}


#[derive(Debug)]
#[non_exhaustive]
pub  enum ErrorKind {
    Bind(io::Error),
    Io(io::Error),
    Runtime(Box<dyn std::error::Error + Send + Sync>),
}


impl Error {
    #[inline(always)]
    pub(crate) fn new(kind: ErrorKind) -> Error { Error { handled: AtomicBool::new(false), kind } }
    #[inline(always)]
    fn was_handled(&self) -> bool {
        self.handled.load(Ordering::Acquire)
    }
    #[inline(always)]
    fn mark_handled(&self) {
        self.handled.store(true, Ordering::Release)
    }
    #[inline]
    pub fn kind(&self) -> &ErrorKind { self.mark_handled();&self.kind }
}


impl std::error::Error for Error {  }
impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.mark_handled();write!(f, "{}", self.kind()) }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error::new(kind)
    }
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { self.mark_handled();self.kind().fmt(f) }
}

impl fmt::Display for ErrorKind {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorKind::Bind(e) => write!(f, "binding failed: {}", e),
            ErrorKind::Io(e) => write!(f, "I/O error: {}", e),
            ErrorKind::Runtime(e) => write!(f, "runtime error: {}", e),
        }
    }
}


impl Drop for Error {
    fn drop(&mut self) {
        if self.was_handled() { return }
        match self.kind() {
            ErrorKind::Bind(ref e) => {println!("{:?}",e)}
            ErrorKind::Io(ref e) => {println!("{:?}",e)}
            ErrorKind::Runtime(ref e) => {println!("{:?}",e)}
        }
    }
}

#[test]
fn xx(){
    println!("{}",ErrorKind::Bind(io::Error::from(io::ErrorKind::Other)));
    println!("{}",ErrorKind::Io(io::Error::from(io::ErrorKind::Other)));
    println!("{}",ErrorKind::Runtime(Box::new(io::Error::from(io::ErrorKind::Other))));
}
