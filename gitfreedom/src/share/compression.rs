use std::{
    fs::{self, File},
    io::{self, BufReader},
    path::PathBuf,
};

use flate2::{bufread::GzDecoder, Compression as FlateCompression, GzBuilder};
use tar::{Archive, Builder};

pub struct Compression;

impl Compression {
    pub fn compress(source: &PathBuf, dest: &PathBuf) -> io::Result<()> {
        // Open the source file
        let mut source_file = File::open(&source)?;

        // Create the output file
        let tar_gz = File::create(&dest)?;

        // Create a GzEncoder on top of the output file
        let enc = GzBuilder::new()
            .mtime(0)
            .write(tar_gz, FlateCompression::default());

        // Create a tar::Builder on top of the encoder
        let mut tar = Builder::new(enc);

        // Append the source file to the tarball
        tar.append_file(source.file_name().unwrap(), &mut source_file)?;

        // Finish the tarball
        tar.finish()?;
        Ok(())
    }

    pub fn decompress(dest: &PathBuf) -> io::Result<()> {
        // Open the tar.gz file
        let tar_gz = File::open(&dest)?;
        let tar_gz = BufReader::new(tar_gz);

        // Create a GzDecoder on top of the file
        let dec = GzDecoder::new(tar_gz);

        // Create a tar::Archive on top of the decoder
        let mut archive = Archive::new(dec);

        // Unpack the archive into the destination directory
        fs::remove_file(&dest)?;
        archive.unpack(dest.parent().unwrap())?;
        Ok(())
    }
}
