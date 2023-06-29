use anyhow::{bail, Context, Ok};
use serde::{Deserialize, Serialize};
use std::io::{StdoutLock, Write};

struct EchoNode {
    id: usize,
}

impl EchoNode {
    fn handle(&mut self, input: Message, output: &mut StdoutLock) -> anyhow::Result<()> {
        match input.body.payload {
            Payload::Echo { echo } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::EchoOk { echo },
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("Failed to serialize message to stdOut")?;
                output.write_all(b"\n").context("new line error")?;
                self.id += 1;
            }
            Payload::Init { .. } => {
                let reply = Message {
                    src: input.dst,
                    dst: input.src,
                    body: Body {
                        id: Some(self.id),
                        in_reply_to: input.body.id,
                        payload: Payload::InitOk,
                    },
                };
                serde_json::to_writer(&mut *output, &reply)
                    .context("Failed to serialize message to stdOut")?;
                output.write_all(b"\n").context("new line error")?;
                self.id += 1;
            }
            Payload::InitOk { .. } => bail!("InitOk is not expected"),
            Payload::EchoOk { .. } => {
                // do nothing.
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Message {
    src: String,
    #[serde(rename = "dest")]
    dst: String,
    body: Body,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Body {
    #[serde(rename = "msg_id")]
    id: Option<usize>,
    in_reply_to: Option<usize>,
    #[serde(flatten)]
    payload: Payload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    InitOk,
}

fn main() -> anyhow::Result<()> {
    let std_in = std::io::stdin().lock();
    let mut std_out = std::io::stdout().lock();
    let messages = serde_json::Deserializer::from_reader(std_in).into_iter::<Message>();
    let mut state = EchoNode { id: 0 };
    for intput in messages {
        let message = intput.context("Failed to deserialize message from stdIn")?;
        state
            .handle(message, &mut std_out)
            .context("Faild to handle message")?
    }
    Ok(())
}
