use anyhow::{Context, Ok};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::io::StdoutLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<Payload> {
    pub src: String,
    #[serde(rename = "dest")]
    pub dst: String,
    pub body: Body<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body<Payload> {
    #[serde(rename = "msg_id")]
    pub id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub payload: Payload,
}

pub trait Node<Payload> {
    fn handle(&mut self, input: Message<Payload>, output: &mut StdoutLock) -> anyhow::Result<()>;
}

pub fn main_loop<S, Payload>(mut state: S) -> anyhow::Result<()>
where
    S: Node<Payload>,
    Payload: DeserializeOwned,
{
    let std_in = std::io::stdin().lock();
    let mut std_out = std::io::stdout().lock();
    let messages = serde_json::Deserializer::from_reader(std_in).into_iter::<Message<Payload>>();

    for intput in messages {
        let message = intput.context("Failed to deserialize message from stdIn")?;
        state
            .handle(message, &mut std_out)
            .context("Faild to handle message")?
    }
    Ok(())
}
