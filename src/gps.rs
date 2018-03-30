//! Bindings to use a GPS chip, and to use it to extract packet metadata.

extern crate time;

use self::time::{Tm,now};

pub struct Data {}

pub fn Enable(_tty_path: String) -> bool {
    return true;
}

pub fn Update() -> Result<Data, String> {
    let res: Data = Data{};
    return Ok(res);
}

pub fn PacketTime(_timestamp: u32) -> Result<Tm, bool> {
    let result: Tm = now();
    Ok(result)
}
