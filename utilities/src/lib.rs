use std::fs::File;
use std::io::BufReader;

/// Returns a `File` handle to the `input.txt` in the working directory.
///
/// # Panics
///
/// Panics by unwrapping any error variant of
/// [`Result`][io::Result][`<File>`][fs::File] returned from 
/// [`File::open`][File::open].
///
/// [fs::File]: https://doc.rust-lang.org/std/fs/struct.File.html
/// [io::Result]: https://doc.rust-lang.org/std/io/type.Result.html
/// [File::open]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
pub fn get_input_file() -> BufReader<File> {
    File::open("input.txt")
        .map(BufReader::new)
        .expect("unable to open input file")
}