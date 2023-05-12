use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;

use super::dto::question::Question;
use super::dto::resource_record::ResourceRecord;


pub static mut CACHE: Cache = Cache{
    hash_map: None,
};

pub struct Cache {
    hash_map: Option<RwLock<HashMap<Question, (ResourceRecord, Instant)>>>,
}

impl Cache {
    pub fn init(&mut self) {
        if self.hash_map.is_none() {
            self.hash_map = Some(RwLock::new(HashMap::new()));
        }
    }

    pub fn get(&self, question: &Question) -> Option<ResourceRecord> {
        let binding = self.hash_map.as_ref().expect("Cache hashmap not initialized");
        let hash_map_reader = binding.read().expect("Cache lock poisoned");
        let result: Option<(ResourceRecord, Instant)> = hash_map_reader.get(&question).cloned();
        drop(hash_map_reader);
        match result {
            None => return None,
            Some((mut rr, instant)) => {
                if rr.ttl as u64 > instant.elapsed().as_secs() {
                    rr.ttl -= instant.elapsed().as_secs() as u32;
                    return Some(rr);
                } else {
                    let mut hash_map_writer = binding.write().expect("Cache lock poisoned");
                    hash_map_writer.remove(&question);
                    return None;
                }
            }
        }
    }

    pub fn insert(&self, question: &Question, rr: ResourceRecord) {
        let binding = self.hash_map.as_ref().expect("Cache hashmap not initialized");
        let mut hash_map_writer = binding.write().expect("Cache lock poisoned");
        hash_map_writer.insert(question.clone(), (rr, Instant::now()));
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    use std::time::Duration;
    use std::thread::sleep;
    use crate::dns::dto::{enums::{TYPE, CLASS}, name::Name};

    fn prepare_test() {
        unsafe { CACHE.init(); }
    }

    #[test]
    fn test_cache_basic() {
        prepare_test();

        let question = Question {
            qname: Name {value: String::from("google.com")},
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };
        let answer = ResourceRecord {
            name: Name {value: String::from("google.com")},
            resource_type: TYPE::A,
            class: CLASS::IN,
            ttl: 10,
            rdlength: 4,
            rdata: vec![8, 8, 8, 8],
        };
        unsafe {CACHE.insert(&question, answer.clone())}

        let reply: Option<ResourceRecord>;
        unsafe {reply = CACHE.get(&question)}
        assert_eq!(reply.unwrap(), answer);
        

        let question = Question {
            qname: Name {value: String::from("bing.com")},
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };

        let reply: Option<ResourceRecord>;
        unsafe {reply = CACHE.get(&question)}
        assert!(reply.is_none());
    }

    #[test]
    fn test_cache_timeout() {
        prepare_test();

        let question = Question {
            qname: Name {value: String::from("google.com")},
            qtype: TYPE::A,
            qclass: CLASS::IN,
        };
        let answer = ResourceRecord {
            name: Name {value: String::from("google.com")},
            resource_type: TYPE::A,
            class: CLASS::IN,
            ttl: 1,
            rdlength: 4,
            rdata: vec![8, 8, 8, 8],
        };
        unsafe {CACHE.insert(&question, answer.clone())}


        let mut reply: Option<ResourceRecord>;
        unsafe {reply = CACHE.get(&question)}
        assert!(reply.is_some());

        sleep(Duration::from_secs(1));

        unsafe {reply = CACHE.get(&question)}
        assert!(reply.is_none());

    }
}