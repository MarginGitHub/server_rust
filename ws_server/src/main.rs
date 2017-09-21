extern crate ws;

use ws::{listen, Handler, Sender, Result, Message, CloseCode};
// use std::rc::Rc;
use std::thread;
use std::io::stdin;

#[derive(Clone)]
struct Server {
    pub out:  Option<Sender>,
}

impl Server {
    // add code here
    pub fn new() -> Self {
        Server{out: None}
    }

    pub fn set_sender(&mut self, sender: Option<Sender>) {
        self.out = sender;
    }

    pub fn get_sender(&mut self) -> Option<&mut Sender> {
        match self.out {
            Some(ref mut sender) => Some(sender),
            None => None
        }
    }
}

impl Handler for Server {
    // add code here

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}", msg);
        Ok(())
        // self.get_sender().unwrap().send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

fn main() {
    let mut server = Server::new();
    let ret = listen("192.168.1.105:3333", |sender|{
        listen_input(sender.clone());
        server.set_sender(Some(sender));
        server.clone()
    });
    match ret {
        Ok(_) => println!("开启成功"),
        Err(err) => println!("{}", err),
    }
}

fn listen_input(sender: Sender) {
    thread::spawn(move || {
        loop {
            let buf = &mut String::new();
            let ret = stdin().read_line(buf);
            match ret {
                Ok(_) => sender.send(Message::text(&buf[..])).unwrap(),
                Err(err) => println!("{:?}", err),
            }
        }
    });
}
