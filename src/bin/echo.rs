use std::io::{StdoutLock, Write};

use anyhow::{bail, Context, Ok};
use serde::{Deserialize, Serialize};

use rustengan::*;

struct EchoNode {
    id: usize,
}

impl Node<Payload> for EchoNode {
    fn handle(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()> {
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
    main_loop(EchoNode { id: 0 })
}
