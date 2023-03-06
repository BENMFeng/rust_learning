use fastx::FastX;
use std::env;
use std::io;
use std::path::Path;
// use core::iter::Iterator;

fn main() -> io::Result<()>
{

    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];
    // for filename in args().skip(1)
    // {
    // println!("{}", filename);
    let mut fastx_reader = FastX::reader_from_path(Path::new(&filename))?;
    let mut fastx_record = FastX::from_reader(&mut fastx_reader)?;

    let mut nb_reads = 0;
    let mut nb_bases = 0;

    // back to serious processing
    while let Ok(_some @ 1..=usize::MAX) = fastx_record.read(&mut fastx_reader)
    {
        // println!("{}\t{}", fastx_record.id(), fastx_record.seq_len())
        nb_reads += 1;
        nb_bases += fastx_record.seq_len();

        // println!("{:?}",fastx_record.id());
    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
    // }
    Ok(())
}