use crate::kv;
use kv::Action;
use rand::Rng;
pub enum Status {
    Follower,
    Candidate,
    Leader
}
pub struct RaftNode{
    log: Vec<(Action, u32)>,
    term_num: u32,
    port: u32,
    timeout: f32,
    status: Status,
    vote: u32
}
impl RaftNode{
    pub fn new(port: u32) -> Self{
        let mut rng = rand::thread_rng();
        RaftNode {
            log: Vec::new(),
            term_num: 0,
            port: port,
            timeout: rng.gen_range(5.0..7.0),
            status: Status::Follower,
            vote: 0,
        }
    }
    pub fn elect(&self, port_list: &Vec<u32>){
        self.staus = Status::Candidate;
        let min_req = (port_list.len() + 1)/2;
        let mut votes = HashSet::new();
        votes.insert(self.port);
        for port in &port_list {
            //find other nodes
        }
        if votes.len() >= min_req {
            self.status = Status::Leader;
        }else{
            self.status = Status::Follower;
        }
    }
}
