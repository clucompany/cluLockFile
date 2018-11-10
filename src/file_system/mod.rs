

mod light;
pub use self::light::*;

use std::io;


#[derive(Debug)]
pub enum ErrFileSysLock {
     LockExists,
     ErrIo(io::Error),
}


