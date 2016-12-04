use std::fs::File;
use std::io::Read;

use errors::*;

pub fn read_file_to_string(filename: &str, target: &mut String) -> Result<()> {
    let mut f = File::open(filename)
                     .chain_err(|| format!("error loading {}", filename))?;
    f.read_to_string(target)
     .chain_err(|| "failed reading day 1 input file")?;
    Ok(())
}
