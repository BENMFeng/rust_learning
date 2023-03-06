use bio::io::fastq;
use std::env;

extern crate bio;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    
    let reader = fastq::Reader::from_file(filename).unwrap();
    let mut nb_reads = 0;
    let mut nb_bases = 0;
    
    for result in reader.records() {

        let result_data = &result.unwrap();

        nb_reads += 1;
        nb_bases += result_data.seq().len();

        // println!("{:?}",result_data.id());

    }

    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
}