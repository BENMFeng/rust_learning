# Thrift rust example

## 0x01 build

```bash
thrift --out src --gen rs simple_service.thrift
cargo build
```



## 0x02 run

```bash
./target/debug/server
./target/debug/client
```



## 0x03 multi client test

```bash
R2-D2 '{"job": "./target/debug/client >1"}' '{"job": "./target/debug/client >2"}' '{"job": "./target/debug/client >3"}' '{"job": "./target/debug/client > 4"}'
```





---

BENM(Binxiao) Feng <binxiaofeng@gmail.com>

2021 Sep 8th
