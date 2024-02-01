use std::{fs, io::{stdout, Write}, time::Duration, char};

use crossterm::{
    queue, terminal::{EnterAlternateScreen, 
    LeaveAlternateScreen, Clear, 
    ClearType, enable_raw_mode, window_size, disable_raw_mode}, 
    event::{
        poll, read, Event, 
        KeyModifiers, KeyCode},
        cursor::{MoveTo, MoveToNextLine}, style::Print
    };

use log::LevelFilter;
use simple_logging::log_to_file;

fn main() {

    let log_file_path = "logs/logs.log"; 

    log_to_file(log_file_path, LevelFilter::Info).unwrap();

    let mut stdout = stdout();

    let mut buffer = Vec::<u8>::new();
    let mut line_endings = Vec::<u16>::new();

    let mut window_width = 0;
    let mut window_height = 0;

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
                        (false, KeyCode::Enter) => {
                            let mut char_buffer: [u8; 4] = [0; 4];
                            '\n'.encode_utf8(&mut char_buffer);

                            for i in 0..'\n'.len_utf8() {
                                buffer.push(char_buffer[i]);
                            }
                        }
                        (_, _) => {
                            // TODO: handle keys
                        }
                    }
                }

                Event::Resize(columns, rows) => {
                    window_width = columns;
                    window_height = rows;
                }
                _ => {
                    // TODO: handle all events
                }
            }


            let mut x:u16 = 0;
            let mut y:u16 = 0;

            queue!(stdout, Clear(ClearType::All)).unwrap();
            queue!(stdout, MoveTo(x, y)).unwrap();
            line_endings.clear();

            let mut i = 0;
            while i < buffer.len() {

                let char_byte = buffer.get(i).unwrap().clone();

                let utf8_len = get_utf8_len_from_first_byte(char_byte);

                // BUG: Bug when rendering non-ascii characters

                let mut utf8_counter = 0;
                let mut utf8_buffer = Vec::<u8>::new();

                while utf8_counter < utf8_len {
                    let byte = buffer.get(i).unwrap().clone();
                    utf8_buffer.push(byte);
                    utf8_counter += 1;
                    i += 1;
                } 

                queue!(stdout, Print(format!("{:?}", utf8_buffer))).unwrap();

                let ch = String::from_utf8(utf8_buffer).unwrap();

                if ch == "\n" {
                    line_endings.push(x);
                    y += 1;
                    queue!(stdout, MoveToNextLine(1)).unwrap();
                    continue;
                }
                queue!(stdout, Print(ch)).unwrap();

                x += 1;
                if x >= window_width {
                    line_endings.push(x);
                    y += 1;
                }

                i += 1;
            }
        }


        stdout.flush().unwrap();
    }

    disable_raw_mode().unwrap();
    queue!(stdout, LeaveAlternateScreen).unwrap();
}

fn get_utf8_len_from_first_byte(byte: u8) -> u8 {
    if (!byte & (1u8 << 7)) != 0 {
        return 1;
    }
    else if (byte & (6u8 << 5)) != 0 {
        return 2;
    }
    else if (byte & (30u8 << 3)) != 0 {
        return 3;
    }
    else {
        return 4;
    }
}
