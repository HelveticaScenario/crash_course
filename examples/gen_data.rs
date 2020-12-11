use rand::Rng;
use std::io::BufWriter;
use std::{env::args, fs::File, io::Write};

fn main() -> std::io::Result<()> {
    let default = 1_000_000u64;
    let count = args()
        .collect::<Vec<_>>()
        .get(1)
        .map(|a| a.parse::<u64>().unwrap_or(default))
        .unwrap_or(default);
    let mut writer = BufWriter::new(File::create("numbers.txt")?);
    let mut rng = rand::thread_rng();
    for _ in 0..count {
        writer.write_fmt(format_args!("{}\n", rng.gen::<u8>()))?;
    }
    Ok(())
}
