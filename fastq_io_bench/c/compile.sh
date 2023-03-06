clang  -Wall -Wextra -Wc++-compat -O2  -mcpu=apple-m1 -DHAVE_KALLOC -I. -o kseq_test kseq_test.c -lz
go build readfq.go
