use input_linux::evdev::EvdevHandle;
use std::{error::Error, result::Result, fmt::{Display, Formatter}};
use std::string::String;
use std::fs::File;


#[derive(Clone, Copy, Debug)]
struct KeyBoardError;

impl Display for KeyBoardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No keyboard detected")
    }
}

impl std::error::Error for KeyBoardError {}

fn find_kbd() -> Result<EvdevHandle<File>, Box<dyn Error>> {
    for n in 1.. {
        let pathstr = format!("/dev/input/event{}", n );
        let eventfile = File::open(pathstr)?;
        let evhandle = EvdevHandle::new(eventfile);

        // check for kb
        // eventfile supports EV_KEY && has repeat settings
        if evhandle.key_mask().is_ok() && evhandle.repeat_settings().is_ok() {
           return Ok(evhandle);
        } else {
           continue;
        }
    }
    return Err(Box::new(KeyBoardError));
}

fn main() -> Result<(), Box<dyn Error>> {
    let nm = find_kbd()?
            .device_name()?;
    let name = String::from_utf8(nm)?;
    
    println!("Device name: {}", &name);
    Ok(())
}
