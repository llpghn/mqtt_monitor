# MQTT-Listener

Einfacher MQTT-Listener der sich mit einem MQTT-Broker verbindet. 
Die erhaltenen Nachrichten werden dann in einer rollierenden Liste 
angezeigt.

Kann über die CLI konfiguriert werden, dann werden aber keine weiteren 
rollierenden Informationen angezeigt.

- Listen
- Configure: 
    Server
      IPv4
    Topics 
      Add / Delete



1. Initalisierungsphase:
  1a. Konfig:Abfrage relevanter Daten wie die IP des zu nutzenden MQTT-Brokers

2. Run-Phase
  2a. __Listener-Thread__ horcht auf ankommende Nachrichten vom Broker
      Über einen Channel werden dann die Nachrichten an einen weiteren Thread
      zur Anzeige gesendet.
  2b. __Anzeige-Thread__ erhält Nachrichten vom Listener-Thread und zeigt die 
      letzten 10 Nachrichten an
  2c. __Input-Thread__ behandelt Benutzereingaben für die Konfiguration der 
      Anwendung.





# backup

    
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
