use anyhow::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::{
        TcpListener, TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub struct JsonRpcServer {
    listener: TcpListener,
    port: u16,
}

impl JsonRpcServer {
    pub async fn bind(addr: &str) -> anyhow::Result<Self> {
        let listener = TcpListener::bind(addr)
            .await
            .with_context(|| format!("Failed to bind on {}", addr))?;

        let addr = listener
            .local_addr()
            .with_context(|| "Failed to get local address")?;
        let port = addr.port();

        Ok(Self { listener, port })
    }

    pub async fn accept(&self) -> anyhow::Result<JsonRpcConnection> {
        let (stream, _) = self
            .listener
            .accept()
            .await
            .with_context(|| "Failed to accept connection")?;

        Ok(JsonRpcConnection::new(stream))
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

pub struct JsonRpcConnection {
    reader: BufReader<OwnedReadHalf>,
    writer: OwnedWriteHalf,
    next_message_id: u64,
}

impl JsonRpcConnection {
    pub fn new(stream: TcpStream) -> Self {
        let (reader, writer) = stream.into_split();

        Self {
            reader: BufReader::new(reader),
            writer,
            next_message_id: 0,
        }
    }

    pub async fn send(&mut self, method: &str, params: Value) -> anyhow::Result<JsonRpcResponse> {
        simplelog::debug!("[RPC_SEND] {}", method);

        let req = JsonRpcRequest {
            jsonrpc: "2.0",
            id: self.message_id(),
            method: method.into(),
            params,
        };

        self.write(&req).await?;
        self.read().await
    }

    async fn write(&mut self, req: &JsonRpcRequest) -> anyhow::Result<()> {
        let body = serde_json::to_string(&req)?;
        let payload = format!("Content-Length: {}\r\n\r\n{}", body.len(), body).into_bytes();

        let _ = self
            .writer
            .write_all(&payload)
            .await
            .with_context(|| "Failed to write payload");

        Ok(())
    }

    async fn read(&mut self) -> anyhow::Result<JsonRpcResponse> {
        let mut content_length: usize = 0;
        let mut header_line = String::new();

        loop {
            header_line.clear();
            self.reader.read_line(&mut header_line).await?;

            let trimmed = header_line.trim();
            if trimmed.is_empty() {
                break;
            }

            if let Some(val) = trimmed.strip_prefix("Content-Length: ") {
                content_length = val.parse()?;
            }
        }

        simplelog::debug!("[RPC_RECV] {} bytes", content_length);

        // Read exact content length
        let mut body = vec![0u8; content_length];
        self.reader.read_exact(&mut body).await?;

        Ok(serde_json::from_slice::<JsonRpcResponse>(&body)?)
    }

    fn message_id(&mut self) -> u64 {
        let message_id = self.next_message_id;
        self.next_message_id += 1;
        message_id
    }
}

#[derive(Debug, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: &'static str,
    pub id: u64,
    pub method: String,
    pub params: Value,
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Option<u64>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}
