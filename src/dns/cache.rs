use std::collections::HashMap;
use std::sync::RwLock;
use std::time::Instant;
use lazy_static::lazy_static;
use fasthash::farm::Hash64 as HasherFn;

use super::dto::question::Question;
use super::dto::resource_record::ResourceRecord;

lazy_static!{
    static ref CACHE: RwLock<HashMap<Question, (ResourceRecord, Instant), HasherFn>> = RwLock::new(HashMap::with_capacity_and_hasher(16, HasherFn));
}

pub fn get(question: &Question) -> Option<ResourceRecord> {
    let hash_map_reader = CACHE.read().expect("Cache lock poisoned");
    let result: Option<(ResourceRecord, Instant)> = hash_map_reader.get(&question).cloned();
    drop(hash_map_reader);
    match result {
        None => return None,
        Some((mut rr, instant)) => {
            let elapsed_time = instant.elapsed().as_secs() as u32;
            let ttl = rr.get_ttl();
            if elapsed_time >= ttl {
                let mut hash_map_writer = CACHE.write().expect("Cache lock poisoned");
                hash_map_writer.remove(&question);
                drop(hash_map_writer);
                return None;
            } else {
                rr.set_ttl(ttl - elapsed_time);
                return Some(rr);
            }
        }
    }
}

pub fn insert(question: &Question, rr: ResourceRecord) {
    let mut hash_map_writer = CACHE.write().expect("Cache lock poisoned");
    hash_map_writer.insert(question.clone(), (rr, Instant::now()));
}

#[allow(dead_code)]
pub fn reset() {
    log::info!("Resetting cache");
    let mut hash = CACHE.write().expect("Cache lock poisoned");
    *hash = HashMap::with_capacity_and_hasher(128, HasherFn);
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::dns::{
        dto::name::Name,
        compression::LabelTree
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
            content: [0, 1, 0, 1],
        };
        let mut answer_byte = Vec::new();
        let mut tree = LabelTree::default();
        let name = Name::from("google.com");
        name.serialize(&mut answer_byte, &mut tree);
        answer_byte.extend_from_slice(&[0, 1, 0, 1, 0, 0, 0, 10, 0, 4, 8, 8, 8, 8]);
        let (answer, _) = ResourceRecord::unserialize(&answer_byte, 0);
        insert(&question, answer.clone());

        let reply: Option<ResourceRecord>;
        reply = get(&question);
        assert_eq!(reply.unwrap(), answer);

        let question = Question {
            qname: Name::from("bing.com"),
            content: [0, 1, 0, 1],
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
            content: [0, 1, 0, 1],
        };
        let mut answer_byte = Vec::new();
        let mut tree = LabelTree::default();
        let name = Name::from("google.com");
        name.serialize(&mut answer_byte, &mut tree);
        answer_byte.extend_from_slice(&[0, 1, 0, 1, 0, 0, 0, 1, 0, 4, 8, 8, 8, 8]);
        let (answer, _) = ResourceRecord::unserialize(&answer_byte, 0);
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
