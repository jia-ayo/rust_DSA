use async_std::{task, net};
use chat::utils::ChatResult;
use std::sync::Arc;
use async_std::prelude::*;
mod connection;
mod chats;
mod chats_map;

use connection::handle;

fn main() -> ChatResult<()>{
    let addr = std::env::args().nth(1)
}
