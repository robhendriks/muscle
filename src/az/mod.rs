pub mod bicep;
pub mod json_rpc;

// use std::path::Path;

// use tokio::io::{AsyncBufReadExt, BufReader};

// #[allow(unused)]
// pub struct BicepClient {}

// impl BicepClient {
//     pub async fn build(_path: impl AsRef<Path>) -> anyhow::Result<()> {
//         let mut child = tokio::process::Command::new("bicep")
//             .args(["build", _path.as_ref().to_str().unwrap()])
//             .stdin(std::process::Stdio::piped())
//             .stdout(std::process::Stdio::piped())
//             .stderr(std::process::Stdio::piped())
//             .spawn()?;

//         let stdout = child
//             .stdout
//             .take()
//             .expect("child did not have a handle to stdout");

//         let mut reader = BufReader::new(stdout).lines();

//         tokio::spawn(async move {
//             let status = child
//                 .wait()
//                 .await
//                 .expect("child process encountered an error");

//             println!("child status was: {}", status);
//         });

//         while let Some(line) = reader.next_line().await? {
//             println!("Line: {}", line);
//         }

//         Ok(())
//     }
// }

// pub struct BicepRpcClient {

// }

// impl BicepRpcClient {

// }
