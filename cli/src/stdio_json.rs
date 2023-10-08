use std::{io::Write, sync::mpsc};

use log::info;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use simulation::TickOutput;

use crate::tabular::tabularize_tick_output;

pub fn stdin_channel<R>() -> mpsc::Receiver<R>
where
    R: Sync + Send + DeserializeOwned + 'static,
{
    let inbound = mpsc::sync_channel(10);

    // receive from stdin
    std::thread::spawn(move || {
        let stdin = std::io::stdin();
        let in_sender = inbound.0;
        let mut buf = String::with_capacity(10_000);
        loop {
            buf.clear();
            stdin.read_line(&mut buf).unwrap();
            let val = serde_json::from_str(&buf).unwrap();
            in_sender.send(val).unwrap();
        }
    });

    return inbound.1;
}

pub fn stdout_channel<S>() -> mpsc::SyncSender<S>
where
    S: Sync + Send + Serialize + 'static,
{
    let outbound = mpsc::sync_channel(10);

    std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let out_recv = outbound.1;
        loop {
            let val = out_recv.recv().unwrap();
            serde_json::to_writer(&mut stdout, &val).unwrap();
            stdout.write_all("\n".as_bytes()).unwrap();
            stdout.flush().unwrap();
        }
    });

    return outbound.0;
}

pub fn tick_output_channel() -> (mpsc::SyncSender<TickOutput>, std::thread::JoinHandle<()>) {
    let outbound = mpsc::sync_channel(10);

    let handle = std::thread::spawn(move || {
        let mut stdout = std::io::stdout();
        let out_recv = outbound.1;
        loop {
            let val = match out_recv.recv() {
                Ok(val) => val,
                Err(e) => {
                    info!("Tick output sender closed: {:?}", e);
                    serde_json::to_writer(&mut stdout, &json!({"done": true})).unwrap();
                    stdout.write_all("\n".as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    return;
                }
            };
            let tabular = tabularize_tick_output(val).unwrap();
            serde_json::to_writer(&mut stdout, &tabular).unwrap();
            stdout.write_all("\n".as_bytes()).unwrap();
            stdout.flush().unwrap();
        }
    });

    return (outbound.0, handle);
}
