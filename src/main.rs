use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    println!("Enter the input file path:");
    let mut input_path = String::new();
    io::stdin().read_line(&mut input_path)?;

    let input_path = PathBuf::from(input_path.trim());

    if !input_path.exists() {
        println!("Input file does not exist.");
        return Ok(());
    }

    let mut input_file = File::open(&input_path)?;

    let input_size = input_file.metadata()?.len();

    println!("Enter the chunk size in MB:");
    let chunk_size_mb = get_input_from_user(0, input_size / (1024 * 1024))?;
    let chunk_size = chunk_size_mb * 1024 * 1024;

    println!("Enter the overlap ratio (0.0 - 1.0):");
    let overlap_ratio = get_input_within_range(0.0, 1.0)?;

    let mut buffer = vec![0; chunk_size as usize];
    let mut count = 0;
    let mut current_offset = 0;

    while current_offset < input_size {
        let output_path = generate_output_path(&input_path, count)?;
        let mut output_file = File::create(&output_path)?;

        input_file.seek(SeekFrom::Start(current_offset))?;

        let mut remaining_bytes = chunk_size as i64;

        while remaining_bytes > 0 {
            let bytes_to_read = std::cmp::min(remaining_bytes as usize, buffer.len());
            let bytes_read = input_file.read(&mut buffer[0..bytes_to_read])?;

            if bytes_read == 0 {
                break;
            }

            output_file.write_all(&buffer[0..bytes_read])?;
            remaining_bytes -= bytes_read as i64;
        }

        count += 1;
        current_offset += (chunk_size as f64 * (1.0 - overlap_ratio)) as u64;
    }

    println!("OK");
    Ok(())
}

fn generate_output_path(input_path: &Path, count: u32) -> io::Result<PathBuf> {
    let file_name = input_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input file name"))?;
    let extension = input_path
        .extension()
        .and_then(|ext| ext.to_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid input file extension"))?;

    let output_file_name = format!("{}_{}.{}", file_name, count, extension);
    let output_path = input_path.with_file_name(output_file_name);

    Ok(output_path)
}

fn get_input_from_user(min: u64, max: u64) -> io::Result<u64> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim().parse::<u64>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")
        })?;

        if value >= min && value <= max {
            return Ok(value);
        } else {
            println!("Input value must be between {} and {}.", min, max);
        }
    }
}

fn get_input_within_range(min: f64, max: f64) -> io::Result<f64> {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let value = input.trim().parse::<f64>().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid input")
        })?;

        if value >= min && value <= max {
            return Ok(value);
        } else {
            println!("Input value must be between {} and {}.", min, max);
        }
    }
}
