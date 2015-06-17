extern crate imap;
extern crate openssl;

use openssl::ssl::{SslContext, SslMethod};
use imap::client::IMAPStream;
use imap::client::IMAPMailbox;

const IMAP_SERVER: &'static str = "imap.gmail.com";
const IMAP_PORT: u16 = 993;
const IMAP_USER: &'static str = "username"
const IMAP_PASS: &'static str = "password";

fn main() {
	let mut imap_socket = match IMAPStream::connect(IMAP_SERVER, IMAP_PORT, Some(SslContext::new(SslMethod::Sslv23).unwrap())) {
		Ok(s) => s,
		Err(e) => panic!("{}", e)
	};

	if let Err(e) = imap_socket.login(IMAP_USER, IMAP_PASS) {
		println!("Error: {}", e)
	};
		
	match imap_socket.capability() {
		Ok(capabilities) => {
			for capability in capabilities.iter() {
				println!("{}", capability);
			}
		},
		Err(_) => println!("Error retreiving capabilities")
	};

	match imap_socket.select("INBOX") {
		Ok(IMAPMailbox{flags, exists, recent, unseen, permanent_flags, uid_next, uid_validity}) => {
			println!("flags: {}, exists: {}, recent: {}, unseen: {:?}, parmanent_flags: {:?}, uid_next: {:?}, uid_validity: {:?}", flags, exists, recent, unseen, permanent_flags, uid_next, uid_validity);
		},
		Err(_) => println!("Error selecting INBOX")
	};

	match imap_socket.namespace() {
		Ok(lines) => {
			for line in lines.iter() {
				print!("{}", line);
			}
		},
		Err(_) => println!("Error reading namespace.")
	};

	match imap_socket.list("\"\" \"%\"") {
		Ok(lines) => {
			for line in lines.iter() {
				print!("{}", line);
			}
		},
		Err(_) => println!("Error listing folders.")
	};

	match imap_socket.fetch("2", "body[text]") {
		Ok(lines) => {
			for line in lines.iter() {
				print!("{}", line);
			}
		},
		Err(_) => println!("Error Fetching email 2")
	};

	if let Err(e) = imap_socket.logout() {
		println!("Error: {}", e)
	};	
}
