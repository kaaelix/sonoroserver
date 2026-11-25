use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SCRAMBLED_BOM: [u8; 3] = [0xA5, 0xF1, 0xF5];
const BUF_SIZE: usize = 64 * 1024;

const DECODE_LUT: [u8; 256] = {
    let mut table = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        table[i] = if (i & 1) != 0 {
            (i as u8) ^ 0xA5
        } else {
            (i as u8) ^ 0xEF
        };
        i += 1;
    }
    table
};

fn default_output_path(input: &Path) -> PathBuf {
    let name = input
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "client.log".to_string());
    let decoded_name = match name.rsplit_once('.') {
        Some((stem, ext)) => format!("{stem}.decoded.{ext}"),
        None => format!("{name}.decoded.log"),
    };
    input.with_file_name(decoded_name)
}

fn decrypt_stream<R: Read, W: Write>(reader: &mut R, writer: &mut W) -> io::Result<u64> {
    let mut buf = vec![0u8; BUF_SIZE];
    let mut first_chunk = true;
    let mut total_written = 0u64;

    loop {
        let n = reader.read(&mut buf)?;
        if n == 0 {
            break;
        }
        let mut slice = &mut buf[..n];
        if first_chunk {
            first_chunk = false;
            if slice.len() >= 3 && slice[..3] == SCRAMBLED_BOM {
                slice = &mut slice[3..];
            }
        }
        for b in slice.iter_mut() {
            *b = DECODE_LUT[*b as usize];
        }
        writer.write_all(slice)?;
        total_written += slice.len() as u64;
    }

    writer.flush()?;
    Ok(total_written)
}

fn run() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!("Usage: decrypt-client-log <input.log> [output.log | -]");
        if args.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "missing input file",
            ));
        }
        return Ok(());
    }
    if args.len() > 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "expected at most 2 arguments: <input.log> [output.log | -]",
        ));
    }

    let input_path = PathBuf::from(&args[0]);
    let in_file = File::open(&input_path)?;
    let mut reader = BufReader::with_capacity(BUF_SIZE, in_file);

    if args.get(1).is_some_and(|s| s == "-") {
        let stdout = io::stdout();
        let mut writer = BufWriter::with_capacity(BUF_SIZE, stdout.lock());
        decrypt_stream(&mut reader, &mut writer)?;
        return Ok(());
    }

    let output_path = args
        .get(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| default_output_path(&input_path));
    let out_file = File::create(&output_path)?;
    let mut writer = BufWriter::with_capacity(BUF_SIZE, out_file);
    let written = decrypt_stream(&mut reader, &mut writer)?;
    eprintln!("wrote {written} bytes to {}", output_path.display());
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn encrypt_sample(plain: &[u8], with_header: bool) -> Vec<u8> {
        let mut out = Vec::new();
        if with_header {
            out.extend_from_slice(&SCRAMBLED_BOM);
        }
        for &b in plain {
            out.push(if (b & 1) != 0 { b ^ 0xEF } else { b ^ 0xA5 });
        }
        out
    }

    #[test]
    fn decodes_header_and_utf8_text() {
        let sample = "[2026.11.25-15.27.40:198][696][GameThread]本地添加buff\n";
        let encrypted = encrypt_sample(sample.as_bytes(), true);
        let mut out = Vec::new();
        let written = decrypt_stream(&mut encrypted.as_slice(), &mut out).unwrap();
        assert_eq!(written as usize, sample.len());
        assert_eq!(out, sample.as_bytes());
    }

    #[test]
    fn decodes_without_header() {
        let sample = b"LogStreaming: Error: Couldn't find file";
        let encrypted = encrypt_sample(sample, false);
        let mut out = Vec::new();
        decrypt_stream(&mut encrypted.as_slice(), &mut out).unwrap();
        assert_eq!(out, sample);
    }
}
