use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {

    // set up hosts and port to be scanned
    let mut hosts: Vec<String> = Vec::new();
    let ports: Vec<u16> = (1..65535).collect();
    for host in 1..255 {
        let address = format!("192.168.1.{}", host);
        hosts.push(address);
    }

    // sync stuff
    let shared_hosts = Arc::new(Mutex::new(hosts));
    let shared_ports = Arc::new(ports);


    // generate x thread to do the parallelly scan
    let handles: Vec<_> = (1..1000).map(|t| {

        let shared_hosts = shared_hosts.clone();
        let shared_ports = shared_ports.clone();

        thread::spawn(move || {

            // pop host from the list, and try to connect to all TCP ports
            loop {
                let mut target_host = None;
                let host = &mut target_host;
                {
                    // do the locking on this scope, so that shared hosts can be unlocked early
                    let mut shared_hosts =  shared_hosts.lock().unwrap();
                    *host = shared_hosts.pop();
                }

                if *host == None {
                    break;
                }

                let target: String = host.as_mut().unwrap().clone();

                for port in &shared_ports[..] {
                    let result = TcpStream::connect((&target[..], *port));
                    match result {
                        Ok(_) => println!("#{}, {}:{} is open", t, target, port),
                        Err(_) => ()
                    }
                }
            }

        })
    }).collect();

    // wait for all threads to finish
    for h in handles {
        h.join().unwrap();
    }

}
