use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::path::Path; // uses path
mod tt;

fn main() {
    println!("Hello, world! Module 'Teletype' successfully loaded!");
    
    // HTTP response to send back to the client
    let stresponse = "";

    // Log messages using your tt module
    tt::teletype("Loaded successfully!");
    tt::teletype("Started Server!");
	let mut html = true;
    // Bind the listener to the address and port
    let listener = TcpListener::bind("0.0.0.0:985").unwrap();
	let file = Path::new("./index.html");
	if file.exists() == true {
		tt::teletype("'index.html' found!");
		html = true;
	} else {
		tt::teletype("'index.html' not found!");
		html = false;
	}
	
    // Loop over incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                tt::teletype("Connection Established!");
                // Write the HTTP response to the stream
                if let Err(e) = stream.write_all(stresponse.as_bytes()) {
                	tt::teletype("Failed to write to stream!");
                }
                // Print the stream info
                println!("{:#?}", stream);
				if html == true {
					serve_html_file(stream,"index.html")
				} else {
					if let Err(e) = stream.write_all("SERVER SIDE ERROR 404; \"index.html\" NOT FOUND".as_bytes()) {
						tt::teletype("Failed to write to stream!");
					}
				}
				                
            }
            Err(e) => {
                tt::teletype("Connection failed!");
            }
        }
    }
}

fn serve_html_file(mut stream: TcpStream, file_path: &str) {
    // Open the HTML file
    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    // Write the HTTP headers
    let stresponse = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n";
    stream.write_all(stresponse.as_bytes()).expect("Failed to write headers");

    // Write the HTML file content line by line
    for line in reader.lines() {
        let line_content = line.expect("Could not read line");
        stream.write_all(line_content.as_bytes()).expect("Failed to write line to stream");
        stream.write_all(b"\n").expect("Failed to write newline to stream");
    }
}
