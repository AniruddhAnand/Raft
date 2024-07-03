use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{Mutex, mpsc};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RaftMessage {
    RequestVote {
        term: u64,
        candidate_id: u64,
    },
    VoteResponse {
        term: u64,
        vote_granted: bool,
    },
    AppendEntries {
        term: u64,
        leader_id: u64,
    },
    AppendEntriesResponse {
        term: u64,
        success: bool,
    },
}

#[derive(Debug, Clone)]
enum NodeState {
    Follower,
    Candidate,
    Leader,
}

pub struct RaftNode {
    id: u64,
    state: Arc<Mutex<NodeState>>,
    current_term: Arc<Mutex<u64>>,
    voted_for: Arc<Mutex<Option<u64>>>,
    votes_received: Arc<Mutex<u64>>,
    leader_id: Arc<Mutex<Option<u64>>>,
    other_nodes: Vec<u64>,
    listener: TcpListener,
}

impl RaftNode {
    pub async fn new(id: u64, port: u32, other_nodes: Vec<u64>) -> Result<Arc<Self>, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
        Ok(Arc::new(Self {
            id,
            state: Arc::new(Mutex::new(NodeState::Follower)),
            current_term: Arc::new(Mutex::new(0)),
            voted_for: Arc::new(Mutex::new(None)),
            votes_received: Arc::new(Mutex::new(0)),
            leader_id: Arc::new(Mutex::new(None)),
            other_nodes,
            listener,
        }))
    }

    pub async fn start(self: Arc<Self>) {
        let (tx, mut rx) = mpsc::channel(100);

        // Spawn a task to handle incoming connections
        let node = self.clone();
        tokio::spawn(async move {
            loop {
                match node.listener.accept().await {
                    Ok((stream, _)) => {
                        let tx_clone = tx.clone();
                        let node_clone = node.clone();
                        tokio::spawn(async move {
                            Self::handle_connection(stream, tx_clone, node_clone).await;
                        });
                    }
                    Err(e) => eprintln!("Error accepting connection: {}", e),
                }
            }
        });

        // Main event loop
        loop {
            tokio::select! {
                Some(message) = rx.recv() => {
                    self.handle_message(message).await;
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    self.start_election().await;
                }
            }
        }
    }

    async fn handle_connection(mut stream: TcpStream, tx: mpsc::Sender<RaftMessage>, node: Arc<RaftNode>) {
        let mut buffer = [0; 1024];
        loop {
            match stream.read(&mut buffer).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    if let Ok(message) = serde_json::from_slice::<RaftMessage>(&buffer[..n]) {
                        if tx.send(message).await.is_err() {
                            break;
                        }
                    }
                }
                Err(_) => break,
            }
        }
    }

    async fn handle_message(&self, message: RaftMessage) {
        match message {
            RaftMessage::RequestVote { term, candidate_id } => {
                let mut current_term = self.current_term.lock().await;
                let mut voted_for = self.voted_for.lock().await;

                if term > *current_term {
                    *current_term = term;
                    *voted_for = None;
                }

                let vote_granted = term >= *current_term && voted_for.is_none();

                if vote_granted {
                    *voted_for = Some(candidate_id);
                }

                self.send_message(candidate_id, RaftMessage::VoteResponse {
                    term: *current_term,
                    vote_granted,
                }).await;
            }
            RaftMessage::VoteResponse { term, vote_granted } => {
                let mut current_term = self.current_term.lock().await;
                if term > *current_term {
                    *current_term = term;
                    *self.state.lock().await = NodeState::Follower;
                } else if vote_granted {
                    let mut votes = self.votes_received.lock().await;
                    *votes += 1;
                    if *votes > (self.other_nodes.len() as u64 + 1) / 2 {
                        *self.state.lock().await = NodeState::Leader;
                        println!("Node {} became leader for term {}", self.id, *current_term);
                    }
                }
            }
            RaftMessage::AppendEntries { term, leader_id } => {
                let mut current_term = self.current_term.lock().await;
                if term >= *current_term {
                    *current_term = term;
                    *self.state.lock().await = NodeState::Follower;
                    *self.leader_id.lock().await = Some(leader_id);
                }

                self.send_message(leader_id, RaftMessage::AppendEntriesResponse {
                    term: *current_term,
                    success: term >= *current_term,
                }).await;
            }
            RaftMessage::AppendEntriesResponse { term, success: _ } => {
                let mut current_term = self.current_term.lock().await;
                if term > *current_term {
                    *current_term = term;
                    *self.state.lock().await = NodeState::Follower;
                }
            }
        }
    }

    async fn start_election(&self) {
        let mut current_term = self.current_term.lock().await;
        *current_term += 1;
        *self.state.lock().await = NodeState::Candidate;
        *self.voted_for.lock().await = Some(self.id);
        *self.votes_received.lock().await = 1;

        for &node in &self.other_nodes {
            self.send_message(node, RaftMessage::RequestVote {
                term: *current_term,
                candidate_id: self.id,
            }).await;
        }
    }

    async fn send_message(&self, target_id: u64, message: RaftMessage) {
        if let Some(&port) = self.other_nodes.iter().find(|&&p| p == target_id) {
            if let Ok(mut stream) = TcpStream::connect(format!("127.0.0.1:{}", port)).await {
                let message_bytes = serde_json::to_vec(&message).unwrap();
                let _ = stream.write_all(&message_bytes).await;
            }
        }
    }
    pub async fn is_leader(&self) -> bool {
        matches!(*self.state.lock().await, NodeState::Leader)
    }
}
