use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

const BUFFER_SIZE: usize = 8192;
const MB: u64 = 1024 * 1024;

struct Config {
    input_path: PathBuf,
    chunk_size: u64,
    overlap_ratio: f64,
}

impl Config {
    fn new() -> io::Result<Self> {
        println!("Enter the input file path:");
        let input_path = get_validated_input_path()?;
        
        let input_size = input_path.metadata()?.len();
        println!("Enter the chunk size in MB (max: {}):", input_size / MB);
        let chunk_size = get_input_from_user(1, input_size / MB)? * MB;

        println!("Enter the overlap ratio (0.0 - 1.0):");
        let overlap_ratio = get_input_within_range(0.0, 1.0)?;

        Ok(Config {
            input_path,
            chunk_size,
            overlap_ratio,
        })
    }
}

fn main() -> io::Result<()> {
    let config = Config::new()?;
    process_file(&config)
}

fn process_file(config: &Config) -> io::Result<()> {
    let input_file = File::open(&config.input_path)?;
    let mut input_reader = BufReader::new(input_file);
    let input_size = input_reader.get_ref().metadata()?.len();

    let total_chunks = calculate_total_chunks(input_size, config.chunk_size, config.overlap_ratio);
    let mut buffer = vec![0; BUFFER_SIZE];
    let mut count = 0;
    let mut current_offset = 0;

    while current_offset < input_size {
        show_progress(count, total_chunks)?;
        process_chunk(
            &mut input_reader,
            &config,
            &mut buffer,
            current_offset,
            count,
        )?;
        
        count += 1;
        current_offset += (config.chunk_size as f64 * (1.0 - config.overlap_ratio)) as u64;
    }

    println!("\nFile splitting completed successfully.");
    println!("Created {} chunks.", count);
    Ok(())
}

fn process_chunk(
    input_reader: &mut BufReader<File>,
    config: &Config,
    buffer: &mut [u8],
    current_offset: u64,
    count: u32,
) -> io::Result<()> {
    let output_path = generate_output_path(&config.input_path, count)?;
    let output_file = File::create(&output_path)?;
    let mut output_writer = BufWriter::new(output_file);

    input_reader.seek(SeekFrom::Start(current_offset))?;

    let mut bytes_written = 0;
    while bytes_written < config.chunk_size {
        let bytes_to_read = std::cmp::min(
            BUFFER_SIZE,
            (config.chunk_size - bytes_written) as usize,
        );
        let bytes_read = input_reader.read(&mut buffer[..bytes_to_read])?;

        if bytes_read == 0 {
            break;
        }

        output_writer.write_all(&buffer[..bytes_read])?;
        bytes_written += bytes_read as u64;
    }

    output_writer.flush()
}

fn get_validated_input_path() -> io::Result<PathBuf> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let path = PathBuf::from(input.trim());

    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Input file does not exist",
        ));
    }
    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Path is not a file",
        ));
    }
    Ok(path)
}

fn show_progress(count: u32, total_chunks: u32) -> io::Result<()> {
    print!("\rProcessing chunk {}/{} ({:.1}%)...", 
           count + 1, 
           total_chunks, 
           (count as f64 + 1.0) / (total_chunks as f64) * 100.0);
    io::stdout().flush()
}

fn calculate_total_chunks(input_size: u64, chunk_size: u64, overlap_ratio: f64) -> u32 {
    ((input_size as f64) / ((chunk_size as f64) * (1.0 - overlap_ratio))).ceil() as u32
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
