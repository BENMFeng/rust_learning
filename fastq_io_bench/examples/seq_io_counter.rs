use seq_io::fastq::{Reader,Record};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    let mut reader = Reader::from_path(filename).unwrap();

    let mut nb_reads = 0;
    let mut nb_bases = 0;

    while let Some(result) = reader.next() {
        let record = result.unwrap();
        nb_reads += 1;
        nb_bases += record.seq().len();

        // println!("{:?}",record.id());
    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
}