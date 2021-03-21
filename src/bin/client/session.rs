use rust_decimal::Decimal;
use feobank::account::{AccountAction, NewAccount};
use feobank::account::AccountAction::*;
use std::net::{SocketAddr, TcpListener, TcpStream, Ipv4Addr};

use feobank::account::Account;

pub struct Session {
    account: Option<Account>,
    conn: TcpStream,
}

impl Session {
    pub fn new(conn: TcpStream) -> Session {
        Session {
            account: None,
            conn
        }
    }
}