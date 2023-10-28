use async_std::io::BuffReader;
use async_std::net::TcpStream;
use async_std::prelude::*;
use async_std::sync::{Arcc, Mutex};
use chat::utils::{self, ChatResult};
use chat::{Client, Server};
use crate::chats_map::ChatTracker;