extern crate thrift;
extern crate example;
use std::thread;
use std::time::Duration;

use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::transport::{TFramedReadTransport, TFramedWriteTransport, TIoChannel, TTcpChannel};

use example::{SimpleServiceSyncClient, TSimpleServiceSyncClient};

fn run(i: i32) {
    println!("spawned thread run {}", i);
    match client(i) {
        Ok(()) => println!("client ran successfully"),
        Err(e) => {
            println!("client failed with error {:?}", e);
            std::process::exit(1);
        }
    }
    
}

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..6 {
            if i % 2 == 0 {
                thread::sleep(Duration::from_millis(2000));
            } else {
                thread::sleep(Duration::from_millis(1));
            }
            run(i);
        }
    });
    let handle2 = thread::spawn(|| {
        for i in 7..12 {
            if i % 2 == 0 {
                thread::sleep(Duration::from_millis(2000));
            } else {
                thread::sleep(Duration::from_millis(1));
            }
            run(i);
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn client(i: i32) -> thrift::Result<()> {
    // build our client and connect to the host:port
    let mut c = TTcpChannel::new();
    c.open("192.168.31.154:9000")?;
    let (i_chan, o_chan) = c.split()?;

    // build the input/output protocol
    let i_prot = TCompactInputProtocol::new(TFramedReadTransport::new(i_chan));
    let o_prot = TCompactOutputProtocol::new(TFramedWriteTransport::new(o_chan));

    // use the input/output protocol to create a Thrift client
    let mut client = SimpleServiceSyncClient::new(i_prot, o_prot);

    // make service calls
    let name = format!("{} {}","BENM", i);
    let res = client.hello(name.to_owned())?;
    println!("{}", res);

    // done!
    Ok(())
}
