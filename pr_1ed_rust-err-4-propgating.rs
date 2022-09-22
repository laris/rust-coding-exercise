let weather = get_weather(hometown)?;

let weather = match get_weather(hometown) {
    Ok(success_value) => success_value;
    Err(err) => return Err(err)
};

let weather = try!(get_weather(hometown));

use std::fs;
use std::io;
use std::path::Path;

fn move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {   // opening dir could fail
        let entry = entry_result?;          // reading dir could fail
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // renaming could fail
    }
    Ok(()) // phew!
}

