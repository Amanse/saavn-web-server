

use tokio::{net::TcpListener, io::{ AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};

#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("localhost:8989").await.unwrap();

	let (tx, _rx) = broadcast::channel(10);

	loop {
		let (mut socket, addr) = listener.accept().await.unwrap();

		let tx = tx.clone();
		let mut rx = tx.subscribe();

		tokio::spawn(async move {
			let (read, mut write) = socket.split();
			let join_msg = format!("{} joined", &addr);
			tx.send((join_msg, addr)).unwrap();

			let mut reader = BufReader::new(read);
			let mut line = String::new();
	
			loop {
				tokio::select! {
					reslt = reader.read_line(&mut line) => {
						if reslt.unwrap() == 0 {
							break;
						}

						line = format!("{}: {}", addr, line);

						tx.send((line.clone(), addr)).unwrap();
						line.clear();
					}
					reslt = rx.recv() => {
						let (msg, other_addr) = reslt.unwrap();
						if addr != other_addr {
							write.write_all(msg.as_bytes()).await.unwrap();
						}

					}
				}
			};
		});
	}
}
