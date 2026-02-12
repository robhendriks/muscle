// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use tokio::{
//     io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
//     net::{
//         TcpListener, TcpStream,
//         tcp::{OwnedReadHalf, OwnedWriteHalf},
//     },
// };

// pub struct Server {
//     listener: TcpListener,
// }

// impl Server {
//     pub async fn new(addr: &str) -> anyhow::Result<Self> {
//         Ok(Self {
//             listener: tokio::net::TcpListener::bind(addr).await?,
//         })
//     }

//     pub async fn accept(&mut self) -> anyhow::Result<Connection> {
//         let (stream, _) = self.listener.accept().await?;
//         Ok(Connection::new(stream))
//     }
// }

// pub struct Connection {
//     reader: BufReader<OwnedReadHalf>,
//     write: OwnedWriteHalf,
// }

// impl Connection {
//     fn new(stream: TcpStream) -> Self {
//         let (read, write) = stream.into_split();

//         Self {
//             reader: BufReader::new(read),
//             write,
//         }
//     }

//     pub async fn request(&mut self, req: &Request) -> anyhow::Result<Response> {
//         let msg = req.encode();
//         self.write.write_all(&msg).await?;
//         Response::decode(&mut self.reader).await
//     }
// }

// #[derive(Debug, Serialize)]
// pub struct Request {
//     pub jsonrpc: &'static str,
//     pub id: u64,
//     pub method: String,
//     pub params: Value,
// }

// impl Request {
//     pub fn encode(&self) -> Vec<u8> {
//         let body = serde_json::to_string::<Self>(self).unwrap();
//         format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes()
//     }
// }

// #[allow(unused)]
// #[derive(Deserialize, Debug)]
// pub struct Response {
//     pub jsonrpc: String,
//     pub id: Option<u64>,
//     pub result: Option<Value>,
//     pub error: Option<Value>,
// }

// impl Response {
//     pub async fn decode(reader: &mut BufReader<OwnedReadHalf>) -> anyhow::Result<Response> {
//         let mut content_length: usize = 0;
//         let mut header_line = String::new();

//         loop {
//             header_line.clear();
//             reader.read_line(&mut header_line).await?;

//             let trimmed = header_line.trim();
//             if trimmed.is_empty() {
//                 break; // End of headers
//             }

//             if let Some(val) = trimmed.strip_prefix("Content-Length: ") {
//                 content_length = val.parse()?;
//             }
//         }

//         // Read exact content length
//         let mut body = vec![0u8; content_length];
//         reader.read_exact(&mut body).await?;

//         let response: Response = serde_json::from_slice(&body)?;
//         Ok(response)
//     }
// }
