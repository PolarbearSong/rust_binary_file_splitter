use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write};

fn main() -> io::Result<()> {
    let chunk_size = 1024 * 1024 * 80;
    let overlap_ratio: f64 = 0.1; // Overlap percentage
    let mut input_file = File::open(r"C:\Binary_File_Splitter\QAR.DAT")?;
    let input_size = input_file.metadata()?.len();
    let mut buffer = vec![0; chunk_size];
    let mut count = 0;
    let mut current_offset = 0;

    while current_offset < input_size {
        let mut output_file = File::create(format!("QAR{}.DAT", count))?;
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
