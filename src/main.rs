use termion::raw::IntoRawMode;
use std::io::{Write, stdout, stdin};
use termion::event::Key;
use termion::input::TermRead;

use std::thread;

use std::sync::mpsc::{self, /*TryRecvError, channel,*/ Sender, Receiver};
use paho_mqtt as mqtt;
// use std::time::Duration;
// use std::io::{self, BufRead};
// use std::sync::{Arc, Mutex};


mod draw_handler;

enum MsgTypes{
    Quit,
    AddThread,
    DelThread,
    Pause, 
    Resume,
}

struct Msg{
    typ: MsgTypes, 
    payload: String
}

fn main() {
    println!("Hello, world for structs!");

    // app variable used to store current state
    let mut server= String::from("");
    let mut connected: bool = false;
    let mut topics: Vec<(u32, String)> = vec![(0, String::from(""))];
    
    topics.pop();
    // Switch to raw mode before entering main-loop
    let mut stdout = stdout().into_raw_mode().unwrap();
    let stdin = stdin();



    
    /* Start thread that handles the MQTT parts */
    let (tx, rx): (Sender<Msg>, Receiver<Msg>) = mpsc::channel();
    let mut received_messages: Vec<(String, String)> = vec![
        (
            String::from("/temp/topicA"), 
            String::from("ValueAAA")
        ),(
            String::from("/temp/topicB"), 
            String::from("ValueBBB")
        ),(
            String::from("/temp/topicC"), 
            String::from("ValueCCC")
        ),
    ];
    
    /* Variable that stores the state if the Thread should print output */
    let mut thread_output: bool = true;
    
    


    // topic = String::from("/sensor/value/temperature");
    // topic = String::from("/sensor/value/humidity");
    // topic = String::from("/sensor/value/pressure");







    thread::spawn(move || loop {
        //println!("Thread-Loop");
        let wrapped_msg_rec = rx.try_recv();
        match wrapped_msg_rec {
            Ok(ref Msg) => {
                let msg_rec = wrapped_msg_rec.unwrap();
                match msg_rec.typ {
                    MsgTypes::Quit => {
                        draw_handler::write_at(24, &String::from("Received quit the programm."));
                        break;
                    },
                    MsgTypes::AddThread => {
                        let a = "Add thread received: ";
                        let result = [a].join(&msg_rec.payload);
                        draw_handler::write_at(24, &result);
                    },
                    MsgTypes::DelThread => {
                        let a = "Delete thread received: ";
                        let result = [a].join(&msg_rec.payload);
                        draw_handler::write_at(24, &result);
                    }
                    MsgTypes::Pause => {
                        let a = "Pause thread received: ";
                        thread_output = false;
                    }
                    MsgTypes::Resume => {
                        let a = "Pause thread received: ";
                        thread_output = true;
                    }
                }
            },
            Err(_) => { }
        };
        /* Display output if thread is running else do nothing */
        if thread_output == true {
            // draw_handler::write_at(23, &String::from("Thread run ..."));
            draw_handler::draw_messages(&received_messages);
        }
    }); /* end thread */


    let event_handling: bool = true;
    if event_handling {

        draw_handler::clear_screen();
        draw_handler::draw_layout();
        draw_handler::draw_static(&server, connected);
        draw_handler::write_at(24, &String::from("Route context: Menue"));
        stdout.flush().unwrap();

        /* Da wir hier nun synchron arbeiten muss der MQTT handler und das 
            schreiben dann dort asynchron erfolgen. */
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => {
                    /* Ending the app */
                    break;
                }, /* end quitting the app*/
                Key::Char('s') => {
                    /* Pause the output from the mqtt-thread */
                    let at = Msg {typ: MsgTypes::Pause, payload: String::from("")};
                    tx.send(at);

                    /* Requesting string from the UI */
                    draw_handler::clear_screen();
                    server = String::from(draw_handler::get_server(&String::from("Server?")));
                    draw_handler::clear_screen();

                    /* Resume the output from the mqtt-thread */
                    let at = Msg {typ: MsgTypes::Resume, payload: String::from("")};
                    tx.send(at);

                }, /* end get server ip addr */
                Key::Char('c') => {
                    if connected == true {
                        connected = false;
                    } else {
                        connected = true;
                        /* Start Thread for listening on MQTT Messages */
                    } /* End switch connect state*/
                    draw_handler::clear_screen();
                }, /* end switch connect to server */
                Key::Char('a') => {
                    /* Pause the output from the mqtt-thread */
                    let at = Msg {typ: MsgTypes::Pause, payload: String::from("")};
                    tx.send(at);

                    draw_handler::clear_screen();
                    let topic = String::from(draw_handler::get_input_string(&String::from("Add Topic:")));
                    topics.push(((topics.iter().count() + 1) as u32, topic));
                    draw_handler::clear_screen();

                    /* Resume the output from the mqtt-thread */
                    let at = Msg {typ: MsgTypes::Resume, payload: String::from("")};
                    tx.send(at);
                    
                }, /* end add topic */
                _ => { /* No key input*/ }
            }

            let at = Msg {typ: MsgTypes::Pause, payload: String::from("")};
            tx.send(at);            
            draw_handler::clear_screen();
            draw_handler::draw_layout();
            draw_handler::draw_static(&server, connected);
            draw_handler::draw_topics(&topics);
            draw_handler::write_at(24, &String::from("Route context: Menue"));
            let at = Msg {typ: MsgTypes::Resume, payload: String::from("")};
            tx.send(at);            
            
            stdout.flush().unwrap();
        } /* end main loop */
    }
    let qt = Msg {typ: MsgTypes::Quit, payload: String::from("")};
    tx.send(qt);
}

