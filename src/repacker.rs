use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub(crate) fn repack_wad(input_dir: &str, output_wad: &str, verbose: bool) -> std::io::Result<()> {
    let mut file_entries: Vec<(String, PathBuf, u64)> = Vec::new();
    collect_file_metadata(Path::new(input_dir), input_dir, &mut file_entries)?;

    file_entries.sort_by(|a, b| a.0.cmp(&b.0));

    let mut output = File::create(output_wad)?;

    let num_files = file_entries.len() as u32;
    output.write_all(&num_files.to_le_bytes())?;

    let mut current_offset = 0u32;
    for (name, _, size) in &file_entries {
        let name_len = name.len() as u32;
        output.write_all(&name_len.to_le_bytes())?;

        output.write_all(name.as_bytes())?;

        let file_size = *size as u32;
        output.write_all(&file_size.to_le_bytes())?;

        output.write_all(&[0u8; 4])?;

        output.write_all(&current_offset.to_le_bytes())?;

        output.write_all(&[0u8; 4])?;

        current_offset += file_size;
    }

    let total = file_entries.len();
    for (i, (name, path, _)) in file_entries.iter().enumerate() {
        let mut file = File::open(path)?;
        std::io::copy(&mut file, &mut output)?;

        if verbose {
            let percent = ((i + 1) as f64 / total as f64 * 100.0) as u32;
            println!("{}/{} ({}%) {}", i + 1, total, percent, name);
        }
    }

    Ok(())
}

fn collect_file_metadata(
    path: &Path,
    base_dir: &str,
    entries: &mut Vec<(String, PathBuf, u64)>,
) -> std::io::Result<()> {
    if path.is_file() {
        let relative_path = path
            .strip_prefix(base_dir)
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");

        let metadata = path.metadata()?;
        let size = metadata.len();

        entries.push((relative_path, path.to_path_buf(), size));
    } else if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            collect_file_metadata(&entry.path(), base_dir, entries)?;
        }
    }

    Ok(())
}
