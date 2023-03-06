use fastq::{parse_path, Record};
use std::env::args;

extern crate fastq;

fn main() {
    let filename = args().nth(1);
    let path = match filename.as_ref().map(String::as_ref) {
        None | Some("-") => None,
        Some(name) => Some(name),
    };

    // parallel_each
    // parse_path(path, |parser| {
    //     let nthreads = 1;
    //     let results: Vec<usize> = parser
    //         .parallel_each(nthreads, |record_sets| {
    //             // let mut thread_total = 0;
    //             let mut nb_reads = 0;
    //             let mut nb_bases = 0;
    //             for record_set in record_sets {
    //                 nb_reads += record_set.len();
    //                 for record in record_set.iter() {
                        // println!("{:?}",String::from_utf8(record.head().to_vec()).unwrap());
    //                     nb_bases += record.seq().len();
    //                 }
    //             }
    //             println!("Number of bases: {}", nb_bases);
    //             nb_reads

    //         })
    //         .expect("Invalid fastq file");
    //     println!("Number of reads: {}", results.iter().sum::<usize>());
    // })
    // .expect("Invalid compression");

    // single thread
    let mut nb_reads = 0;
    let mut nb_bases = 0;
    let mut nb_qualities  = 0;
    parse_path(path, |parser| {
        parser
            .each(|record| {
                nb_reads += 1;
                nb_bases += record.seq().len();
                nb_qualities += record.qual().len();
                // println!("{:?}",String::from_utf8(record.head().to_vec()).unwrap());
                true
            })
            .expect("Invalid fastq file");
    })
    .expect("Invalid compression");
    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
    println!("Number of qualities: {}", nb_qualities);

}