use std::{
    env,
    error::Error,
    fs::{self, OpenOptions},
    io::{self, Cursor, Write},
};
use zip::ZipWriter;
use zip_extensions::ZipWriterExtensions;

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();
    let [ref executable_path, ref action, ref directory_path] = args[..] else {
        return Err("Usage: <action> <directory>".into());
    };
    match action.as_str() {
        "box" => zip_and_append(executable_path, directory_path)?,
        "unbox" => {
            fs::create_dir_all(directory_path)?;
            zip_extensions::zip_extract(&executable_path.into(), &directory_path.into())?;
        }
        _ => return Err("Invalid action. Use 'box' or 'unbox'.".into()),
    }
    Ok(())
}

fn zip_and_append(executable_path: &str, directory_path: &str) -> io::Result<()> {
    let mut zip_content = Vec::new();
    let zip = ZipWriter::new(Cursor::new(&mut zip_content));
    zip.create_from_directory(&directory_path.into())?;
    let mut executable_file = OpenOptions::new().append(true).open(executable_path)?;
    executable_file.write_all(&zip_content)
}
