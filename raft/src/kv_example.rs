/*mod kv;
pub use kv::kv_store::KV;
pub use kv::kv_store::Action;
fn main(){
    let path = "/Users/anianand/DB/db1.db";
    let log_path = "/Users/anianand/DB/db_log_1.log";
    let mut new_db: KV = KV::new(path, log_path);
    let _ = new_db.act(Action::Write {key: 10, value: 20});
    let _ = new_db.act(Action::Read {key: 10});
    new_db.save();
    let _ = new_db.act(Action::Delete {key: 10});
    new_db.save();
}*/
