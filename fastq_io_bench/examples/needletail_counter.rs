extern crate needletail;
use std::env;
use std::error::Error;
use needletail::parse_fastx_file;

fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();
    let filename: &str = &args[1];

    // let mut reader = needletail::parse_fastx_file(filename)?;

    let mut nb_reads = 0;
    let mut nb_bases = 0;
    let mut nb_qualities  = 0;
    // let mut n_valid_kmers = 0;
    let mut reader = parse_fastx_file(&filename).expect("valid path/file");
    while let Some(record) = reader.next() {
        let seqrec = record.expect("invalid record");
        // keep track of the total number of bases
        nb_bases += seqrec.num_bases();
        nb_qualities += seqrec.qual().unwrap().len();
        nb_reads += 1;

        // normalize to make sure all the bases are consistently capitalized and
        // that we remove the newlines since this is FASTA
        // let norm_seq = seqrec.normalize(false);
        // we make a reverse complemented copy of the sequence first for
        // `canonical_kmers` to draw the complemented sequences from.
        // let rc = norm_seq.reverse_complement();
        // now we keep track of the number of AAAAs (or TTTTs via
        // canonicalization) in the file; note we also get the position (i.0;
        // in the event there were `N`-containing kmers that were skipped)
        // and whether the sequence was complemented (i.2) in addition to
        // the canonical kmer (i.1)
        // for (_, kmer, _) in norm_seq.canonical_kmers(4, &rc) {
        //     if kmer == b"AAAA" {
        //         n_valid_kmers += 1;
        //     }
        // }
    }
    // println!("There are {} bases in your file.", n_bases);
    // println!("There are {} AAAAs in your file.", n_valid_kmers);
    println!("Number of reads: {}", nb_reads);
    println!("Number of bases: {}", nb_bases);
    println!("Number of qualities: {}", nb_qualities);
    Ok(())
}