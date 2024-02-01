use std::{fs, io::{stdout, Write}, time::Duration};

use crossterm::{
    queue, terminal::{EnterAlternateScreen, 
    LeaveAlternateScreen, Clear, 
    ClearType, enable_raw_mode}, 
    event::{
        poll, read, Event, 
        KeyModifiers, KeyCode},
        cursor::MoveTo, style::Print
    };

use log::LevelFilter;
use simple_logging::log_to_file;

fn main() {

    let log_file_path = "logs/logs.log"; 

    log_to_file(log_file_path, LevelFilter::Info).unwrap();

    let mut stdout = stdout();

    let mut buffer = Vec::<u8>::new();

    let mut data = fs::read("test/test.txt").unwrap();

    buffer.append(&mut data);

    let data_string = String::from_utf8(data).unwrap();

    log::info!("File {} read: {}", "test/test.txt", data_string);

    enable_raw_mode().unwrap();

    queue!(stdout, EnterAlternateScreen).unwrap();
    queue!(stdout, Clear(ClearType::All)).unwrap();
    queue!(stdout, MoveTo(0, 0)).unwrap();
    queue!(stdout, Print(data_string)).unwrap();


    stdout.flush().unwrap();
    
    let mut is_running = true;

    while is_running {
        if poll(Duration::from_millis(500)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    let is_control = event.modifiers.contains(KeyModifiers::CONTROL);
                    let key = event.code;

                    match (is_control, key) {
                        (true, KeyCode::Char('c')) => {
                            is_running = false;
                        }
                        (true, KeyCode::Char('s')) => {
                            fs::write("test/test.txt", String::from_utf8(buffer.clone()).unwrap()).unwrap();
                        }
                        (false, KeyCode::Char(c)) => {
                            let mut char_buffer: [u8; 4] = [0; 4];
                            c.encode_utf8(&mut char_buffer);

                            for i in 0..c.len_utf8() {
                                buffer.push(char_buffer[i]);
                            }
                        }
                        (_, _) => {
                            // TODO: handle keys
                        }
                    }
                }
                _ => {;
                    // TODO: handle all events
                }
            }

            let text_to_render = String::from_utf8(buffer.clone()).unwrap();

            queue!(stdout, Clear(ClearType::All)).unwrap();
            queue!(stdout, MoveTo(0, 0)).unwrap();
            queue!(stdout, Print(text_to_render)).unwrap();
        }


        stdout.flush().unwrap();
    }

    queue!(stdout, LeaveAlternateScreen).unwrap();
}
