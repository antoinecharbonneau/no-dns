use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;
use lazy_static::lazy_static;

use super::dto::question::Question;
use super::dto::resource_record::ResourceRecord;

lazy_static!{
    static ref CACHE: RwLock<HashMap<Question, (ResourceRecord, Instant)>> = RwLock::new(HashMap::new());
}

pub fn get(question: &Question) -> Option<ResourceRecord> {
    let hash_map_reader = CACHE.read().expect("Cache lock poisoned");
    let result: Option<(ResourceRecord, Instant)> = hash_map_reader.get(&question).cloned();
    drop(hash_map_reader);
    match result {
        None => return None,
        Some((mut rr, instant)) => {
            let elapsed_time = instant.elapsed().as_secs() as u32;
            if elapsed_time >= rr.ttl {
                let mut hash_map_writer = CACHE.write().expect("Cache lock poisoned");
                hash_map_writer.remove(&question);
                drop(hash_map_writer);
                return None;
            } else {
                rr.ttl -= elapsed_time;
                return Some(rr);
            }
        }
    }
}

pub fn insert(question: &Question, rr: ResourceRecord) {
    let mut hash_map_writer = CACHE.write().expect("Cache lock poisoned");
    hash_map_writer.insert(question.clone(), (rr, Instant::now()));
}

pub fn reset() {
    log::info!("Resetting cache");
    let mut hash = CACHE.write().expect("Cache lock poisoned");
    *hash = HashMap::new();
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::dns::dto::{
        enums::{CLASS, TYPE},
        name::Name,
    };
    use std::thread::sleep;
    use std::time::Duration;
    use std::sync::Mutex;
    use std::marker::PhantomData;
    
    static MUTEX: Mutex<PhantomData<()>> = Mutex::new(PhantomData {});

    #[test]
    fn test_cache_basic() {
        let _lock = MUTEX.lock();
        reset();
        let question = Question {
            qname: Name::from("google.com"),
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };
        let answer = ResourceRecord {
            name: Name::from("google.com"),
            resource_type: TYPE::A,
            class: CLASS::IN,
            ttl: 10,
            rdlength: 4,
            rdata: vec![8, 8, 8, 8],
        };
        insert(&question, answer.clone());

        let reply: Option<ResourceRecord>;
        reply = get(&question);
        assert_eq!(reply.unwrap(), answer);

        let question = Question {
            qname: Name::from("bing.com"),
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };

        let reply: Option<ResourceRecord>;
        reply = get(&question);
        assert!(reply.is_none());
    }

    #[test]
    fn test_cache_timeout() {
        let _lock = MUTEX.lock();
        reset();
        let question = Question {
            qname: Name::from("google.com"),
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };
        let answer = ResourceRecord {
            name: Name::from("google.com"),
            resource_type: TYPE::A,
            class: CLASS::IN,
            ttl: 1,
            rdlength: 4,
            rdata: vec![1, 2, 3, 4],
        };
        insert(&question, answer.clone());

        let mut reply: Option<ResourceRecord>;
        reply = get(&question);
        assert!(reply.is_some());

        sleep(Duration::from_millis(1010));

        reply = get(&question);
        println!("{:?}", reply);
        assert!(reply.is_none());
    }
}
