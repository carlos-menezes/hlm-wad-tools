use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

pub(crate) fn extract_wad(filename: &str, output_dir: &str, verbose: bool) -> std::io::Result<()> {
    fs::create_dir_all(output_dir)?;

    let mut file = File::open(filename)?;

    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    let num_files = u32::from_le_bytes(buffer);

    let mut entries = Vec::new();

    for _ in 0..num_files {
        file.read_exact(&mut buffer)?;
        let name_len = u32::from_le_bytes(buffer) as usize;

        let mut name_bytes = vec![0u8; name_len];
        file.read_exact(&mut name_bytes)?;
        let name = String::from_utf8_lossy(&name_bytes).to_string();

        file.read_exact(&mut buffer)?;
        let file_size = u32::from_le_bytes(buffer);

        file.read_exact(&mut buffer)?;

        file.read_exact(&mut buffer)?;
        let file_offset = u32::from_le_bytes(buffer);

        file.read_exact(&mut buffer)?;

        entries.push((name, file_size, file_offset));
    }

    let table_end = file.stream_position()?;
    let total = entries.len();

    for (i, (name, size, offset)) in entries.iter().enumerate() {
        let actual_offset = table_end + (*offset as u64);

        let filepath = Path::new(output_dir).join(name);
        if let Some(parent) = filepath.parent() {
            fs::create_dir_all(parent)?;
        }

        file.seek(SeekFrom::Start(actual_offset))?;
        let mut out_file = File::create(&filepath)?;
        std::io::copy(
            &mut Read::by_ref(&mut file).take(*size as u64),
            &mut out_file,
        )?;

        if verbose {
            let percent = ((i + 1) as f64 / total as f64 * 100.0) as u32;
            println!("{}/{} ({}%) {}", i + 1, total, percent, name);
        }
    }

    Ok(())
}
