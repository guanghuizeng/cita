// CITA
// Copyright 2016-2017 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.
extern crate kafka;

use std::time::Duration;

use kafka::producer::{Producer,Record,RequiredAcks};
use kafka::consumer::{Consumer,FetchOffset,GroupOffsetStorage};
use kafka::error::Error as KafkaError;

use::std::sync::mpsc::Receiver;
use::std::sync::mpsc::Sender;
use std::thread;

pub fn start_kafka(name: &str, keys: Vec<&str>, tx: Sender<(String,Vec<u8>)>, rx: Receiver<(String,Vec<u8>)>){
    println!("into start_kafka");
    let  _=thread::Builder::new().name("publisher".to_string()).spawn(move || {
        println!("producer thread running!");
        let broker = "localhost:9092";
        loop {
            let mut ret = rx.recv();
            if ret.is_err(){
                break;
            }
            let (topic, msg) = ret.unwrap();
            if let Err(e) = produce_message(&msg, &topic, vec![broker.to_string()]) {
                println!("Failed producing messages: {}", e);
            }
        }
    });
    let mut topics = Vec::new();
    for key in keys {
        topics.push(key.to_string());
    }
    let tx = tx.clone();
    //comsumer thread
    let _ = thread::Builder::new().name("consumer".to_string()).spawn(move ||{
        println!("consumer thread running!");
        let mut con = Consumer::from_hosts(vec!["localhost:9092".to_owned()])
                        .with_fallback_offset(FetchOffset::Earliest);
                        
        for topic in topics {
            con = con.with_topic(topic);
        }
        let mut con = con.create().unwrap();
        loop{
            let mss = con.poll().unwrap();
            if mss.is_empty(){
                println!("No Messages available right now.");
            }
            println!("poll Messages.");
            for ms in mss.iter(){
                for m in ms.messages(){
                    println!("get msg {:?} {:?}",ms.topic(), m.value);
                    tx.send((ms.topic().to_string(), m.value.to_vec())).unwrap();
                }
                let _ = con.consume_messageset(ms);
            
            }
            con.commit_consumed().unwrap();
        }
    });
}

fn produce_message<'a, 'b>(data: &'a [u8],
                           topic: &'b str,
                           brokers: Vec<String>)
                           -> Result<(), KafkaError> {
    println!("About to publish a message at {:?} to: {} data {:?}", brokers, topic, data);

    // ~ create a producer. this is a relatively costly operation, so
    // you'll do this typically once in your application and re-use
    // the instance many times.
    let mut producer = try!(Producer::from_hosts(brokers)
             // ~ build the producer with the above settings
             .create());

    // ~ now send a single message.  this is a synchronous/blocking
    // operation.

    // ~ we're sending 'data' as a 'value'. there will be no key
    // associated with the sent message.

    // ~ we leave the partition "unspecified" - this is a negative
    // partition - which causes the producer to find out one on its
    // own using its underlying partitioner.

    // ~ we can achieve exactly the same as above in a shorter way with
    // the following call
    try!(producer.send(&Record::from_value(topic, data)));

    Ok(())
}
