//for now each each log action is an action db operation
//all queries must go through leader
use std::vec::Vec;
pub mod raft_node {
    pub enum Action {
        Create {key: u32, val: u32},
        Read {key: u32},
        Write {key: u32, val: u32},
        Delete {key: u32}
    }
    pub struct Log {
        id: u32,
        cur: Action,
        prev: Option<Box<Log>>,
        next: Option<Box<Log>>
    }
    //will assume all nodes are in the same cluster
    pub struct Node {
        log_head: Option<Log>,
        cur: Option<Action>,
        is_leader: bool,
        port: u32
    }
    impl Node{
        fn new(port: u32) -> Node {
            Node{
                log_head: None,
                cur: None,
                is_leader: false,
                port: port
            }
        }
    }
}
