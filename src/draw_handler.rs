use termion::{raw::IntoRawMode, cursor, clear, style};
use std::io::{Write, stdout, stdin};
//use std::str;

use termion::event::Key;

use termion::input::TermRead;


/// Draws message into the corresponding box
///
/// # Arguments
///
/// * `msgs` - &Vec<(String, String) - Contains the messages that will be 
///                                     printed
pub fn draw_messages(msgs: &Vec<(String, String)>){
  let mut draw_offset: u16 = 0;
  for msg in msgs{
    print!("{goto}", goto = cursor::Goto(33, 9 + draw_offset));
    if draw_offset >= 13 {
      print!("(...)");
      continue;
    }
    print!("{} \n\t {}", msg.0, msg.1);
    draw_offset += 2;
  }
}


/// Draws a box defined by the parameters
///
/// # Arguments
///
/// * `tl_x` - top left x position
/// * `tl_y` - top left y position
/// * `br_x` - buttom right x position
/// * `br_y` - buttom right y position
fn draw_box(tl_x: u16, tl_y: u16, br_x: u16, br_y: u16){
  // TODO:  Wir zeichnen nur den Rahmen aber kein Box-Objekt damit 
  //        würden die hinteren Layer durchscheinen können :/
  let mut horizontal: u16 = tl_x + 1;
  let mut vertically: u16 = tl_y + 1;

  print!("{goto}╔", goto = cursor::Goto(tl_x, tl_y));
  print!("{goto}╗", goto = cursor::Goto(br_x, tl_y));
  print!("{goto}╚", goto = cursor::Goto(tl_x, br_y));
  print!("{goto}╝", goto = cursor::Goto(br_x, br_y));
  while horizontal < br_x {
    print!("{goto}═", goto = cursor::Goto(horizontal, tl_y));
    print!("{goto}═", goto = cursor::Goto(horizontal, br_y));
    horizontal += 1;
  }
  while vertically < br_y {
    print!("{goto}║", goto = cursor::Goto(tl_x, vertically));
    print!("{goto}║", goto = cursor::Goto(br_x, vertically));
    vertically += 1;
  }
}

/// Draws partially static content of the app like the upper menue
///
/// # Arguments
///
/// * `ip` - String - represents the ip the app connects to
/// * `connected` - bool - Displays if the app is connected to the server
pub fn draw_static(ip: &String, connected: bool){
  let mut con = String::from("");
  if connected == true {
    con.push('X');
  } else {
    con.push(' ');
  }
  print!("{goto}{underline}S{reset}erver: [{ip}]", goto = cursor::Goto(3, 3),
                                                              underline = style::Underline,
                                                              reset = style::Reset, 
                                                              ip = ip);
  print!("{goto}{underline}C{reset}onnected: [{con}]",  goto = cursor::Goto(35, 3),     
                                                    underline = style::Underline,
                                                    reset = style::Reset,
                                                    con = con);
  print!("{goto}{underline}A{reset}dd topic", goto = cursor::Goto(3, 5),
                                              underline = style::Underline,
                                              reset = style::Reset);
  print!("{goto}{underline}D{reset}elete topic",  goto = cursor::Goto(25, 5),
                                                  underline = style::Underline,
                                                  reset = style::Reset);
  print!("{goto}{underline}Q{reset}uit",  goto = cursor::Goto(55, 5),
                                          underline = style::Underline,
                                          reset = style::Reset);
}

/// Receives a list of topics to display
///
/// # Arguments
///
/// * `topics` - &Vec<(u32, String)> - List of topics to display in the lower
///                                    left panel of the app.
pub fn draw_topics(topics: &Vec<(u32, String)>){
  let mut draw_offset: u16 = 0;
  for topic in topics{
    print!("{goto}", goto = cursor::Goto(3,9 + draw_offset));
    if draw_offset >= 13 {
      print!("(...)");
      continue;
    }
    print!("{} {}", topic.0, topic.1);
    draw_offset += 1;
  }
}

/// Clears part of a line
/// # Arguments
/// * `line` - Index of the line that will be cleared
/// * `cursor` - x-Axis start character to clear from
/// * `length` - How many characters will be cleared 
pub fn clear_line_part(line: u16, cursor: u16, length: usize){
  let clear_string = std::iter::repeat(" ").take(length).collect::<String>();
  print!("{goto}", goto = cursor::Goto(cursor, line));
  print!("{}", clear_string);
}

pub fn write_at(line: u16, text: &String){
  print!("{goto}", goto = cursor::Goto(1, line));
  print!("{}", termion::clear::CurrentLine);
  print!("{}", text);
}


/// Reads a string while beeing in raw terminal mode
///
/// # Arguments
/// * `write_row` - Where will the User Output be displayed
/// * `write_start` - Cursor column position
/// * `max_length` - Max length of read string
fn read_string_from_tty(write_row: u16, write_col: u16, length: usize) -> String{
  let stdin = stdin();
  let mut stdout = stdout().into_raw_mode().unwrap();
  stdout.flush().unwrap();
  let mut in_buffer = String::from("");

  print!("{goto}", goto = cursor::Goto(write_row, write_col));

  for c in stdin.keys() {
    clear_line_part(write_row, write_col, length);
    write!(stdout, "{}", termion::cursor::Goto(write_col, write_row)).unwrap();
    match c.unwrap() {
      Key::Char('\n') => { return in_buffer },
      Key::Backspace => { in_buffer.pop(); {} },
      Key::Char('/') => in_buffer.push('/'),
      Key::Char('.') => in_buffer.push('.'),
      Key::Char(':') => in_buffer.push(':'),
      Key::Char('1') => in_buffer.push('1'),
      Key::Char('2') => in_buffer.push('2'),
      Key::Char('3') => in_buffer.push('3'),
      Key::Char('4') => in_buffer.push('4'),
      Key::Char('5') => in_buffer.push('5'),
      Key::Char('6') => in_buffer.push('6'),
      Key::Char('7') => in_buffer.push('7'),
      Key::Char('8') => in_buffer.push('8'),
      Key::Char('9') => in_buffer.push('9'),
      Key::Char('0') => in_buffer.push('0'),
        
      Key::Char('a') | Key::Char('A') => in_buffer.push('a'),
      Key::Char('b') | Key::Char('B') => in_buffer.push('b'),
      Key::Char('c') | Key::Char('C') => in_buffer.push('c'),
      Key::Char('d') | Key::Char('D') => in_buffer.push('d'),
      Key::Char('e') | Key::Char('E') => in_buffer.push('e'),
      Key::Char('f') | Key::Char('F') => in_buffer.push('f'),
      Key::Char('g') | Key::Char('G') => in_buffer.push('g'),
      Key::Char('h') | Key::Char('H') => in_buffer.push('h'),
      Key::Char('i') | Key::Char('I') => in_buffer.push('i'),
      Key::Char('j') | Key::Char('J') => in_buffer.push('j'),
      Key::Char('k') | Key::Char('K') => in_buffer.push('k'),
      Key::Char('l') | Key::Char('L') => in_buffer.push('l'),
      Key::Char('m') | Key::Char('M') => in_buffer.push('m'),
      Key::Char('n') | Key::Char('N') => in_buffer.push('n'),
      Key::Char('o') | Key::Char('O') => in_buffer.push('o'),
      Key::Char('p') | Key::Char('P') => in_buffer.push('p'),
      Key::Char('q') | Key::Char('Q') => in_buffer.push('q'),
      Key::Char('r') | Key::Char('R') => in_buffer.push('r'),
      Key::Char('s') | Key::Char('S') => in_buffer.push('s'),
      Key::Char('t') | Key::Char('T') => in_buffer.push('t'),
      Key::Char('u') | Key::Char('U') => in_buffer.push('u'),
      Key::Char('v') | Key::Char('V') => in_buffer.push('v'),
      Key::Char('w') | Key::Char('W') => in_buffer.push('w'),
      Key::Char('x') | Key::Char('X') => in_buffer.push('x'),
      Key::Char('y') | Key::Char('Y') => in_buffer.push('y'),
      Key::Char('z') | Key::Char('Z') => in_buffer.push('z'),
        
      _ => {}
    }
    
    print!("{}",in_buffer);
    stdout.flush().unwrap();
  }
  in_buffer

}

/// Can be used to query the server IP-Adress from the user
///
/// # Arguments
///
/// * `question` - A custom Question that can be displayed to the user.
pub fn get_server(question: &String) -> String{
  print!("{}", termion::clear::All);
  draw_box(10, 9 , 70, 15);
  print!("{goto}", goto = cursor::Goto(12, 11));
  print!("{}", question);

  let server_address = read_string_from_tty(13, 12, 50);
  server_address
}

pub fn get_input_string(question: &String) -> String{
  print!("{}", termion::clear::All);
  draw_box(10, 9 , 70, 15);
  print!("{goto}", goto = cursor::Goto(12, 11));
  print!("{}", question);

  let input_string = read_string_from_tty(13, 12, 50);
  input_string
}

/// Draws the layout of the app (better the boxes that defines the layout)
pub fn draw_layout(){
  draw_box(1, 1, 80, 7);    // top-box with commandos
  draw_box(1, 8, 30, 23);   // left-lower-box with topics
  draw_box(31, 8, 80, 23);  // right-lower-box with messages
}

pub fn clear_screen(){
  println!("{}", clear::All);
}
