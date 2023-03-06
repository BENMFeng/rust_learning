# Fastq IO Benchmark

Repo: GitLab:fengbinxiao/rust_learning/tree/master/fastq_io_bench

大牛如是说：

* [X] fastq="0.6.0"
* [X] bio="1.1.0"
* [X] seq="0.3.1"
* [X] fastx="0.2.0"
* [X] needletail="0.5.0"
* [X] readfq (zlib, kseq)



Example:

```
cargo run --example fastq_counter data/test.fastq

cargo run --example bio_counter data/test.fastq

cargo run --example seq_io_counter data/test.fastq
```


Read bench:

|          | ramdisk | lz4     | lz4 + thread | gzip    | gzip + thread |
| -------- | ------- | ------- | ------------ | ------- | ------------- |
| wc -l    | 2.3GB/s | 1.2GB/s | NA           | 300MB/s | NA            |
| fastq    | 1.9GB/s | 1.9GB/s | 1.6GB/s      | 650MB/s | 620MB/s       |
| rust-bio | 730MB/s | NA      | 250MB/s      | NA      | NA            |
| seqan    | 150MB/s | NA      | NA           | NA      | NA            |
| kseq.h   | 980MB/s | 680MB/s | NA           | NA      | NA            |

```Shell
du -shc ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
11G        ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
11G        total

time cargo run --example fastq_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
   Compiling fastq_io_bench v0.1.0 (fastq_io_bench)
    Finished dev [unoptimized + debuginfo] target(s) in 1.06s
     Running `target/debug/examples/fastq_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        0m33.617s
user        0m31.449s
sys        0m2.116s

time cargo run --example fastq_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/examples/fastq_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        0m32.675s
user        0m30.848s
sys        0m1.841s

time cargo run --example bio_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
   Compiling fastq_io_bench v0.1.0 (/mnt/gpfs/Users/fengbinxiao/bioinfor/04.jedi/fastq_io_bench)
    Finished dev [unoptimized + debuginfo] target(s) in 1.88s
     Running `target/debug/examples/bio_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        2m0.814s
user        1m56.220s
sys        0m4.563s

time cargo run --example bio_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/bio_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        1m58.017s
user        1m53.877s
sys        0m4.185s

time cargo run --example seq_io_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
   Compiling fastq_io_bench v0.1.0 (/mnt/gpfs/Users/fengbinxiao/bioinfor/04.jedi/fastq_io_bench)
    Finished dev [unoptimized + debuginfo] target(s) in 0.85s
     Running `target/debug/examples/seq_io_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        0m36.263s
user        0m33.919s
sys        0m2.462s

time cargo run --example seq_io_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/examples/seq_io_counter ../../09.hg38_fmindex/Read1-ARS220113BF018.fq`
Number of reads: 22520887
Number of bases: 1735766438

real        0m35.532s
user        0m33.303s
sys        0m2.245s

time wc -l ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
90083548 ../../09.hg38_fmindex/Read1-ARS220113BF018.fq

real        0m2.652s
user        0m0.978s
sys        0m1.678s
time wc -l ../../09.hg38_fmindex/Read1-ARS220113BF018.fq
90083548 ../../09.hg38_fmindex/Read1-ARS220113BF018.fq

real        0m2.251s
user        0m1.036s
sys        0m1.219s
```

1M gz

```Shell
time cargo run --example needletail_counter data/test_1M.fq.gz
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/needletail_counter data/test_1M.fq.gz`
Number of reads: 1000000
Number of bases: 74721963
Number of qualities: 74721963
cargo run --example needletail_counter data/test_1M.fq.gz  2.06s user 0.04s system 94% cpu 2.217 total

time cargo run --example fastq_counter data/test_1M.fq.gz
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/examples/fastq_counter data/test_1M.fq.gz`
Number of reads: 1000000
Number of bases: 74721963
Number of qualities: 74721963
cargo run --example fastq_counter data/test_1M.fq.gz  1.93s user 0.05s system 141% cpu 1.396 total

cat ../data/test_1M.fq.gz |time ./kseq_test
1000000        74721963        74721963
./kseq_test  0.63s user 0.01s system 99% cpu 0.645 total.501 total
```


参考：

**http://lh3.github.io/2020/05/17/fast-high-level-programming-languages**

---
