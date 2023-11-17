mod file;

use std::collections::HashMap;
use fasthash::city as Hasher;
use lazy_static::lazy_static;

use crate::dns::dto::label::Label;

lazy_static! {
    static ref BLOCKLIST: Blocklist = Blocklist::init();
}

pub struct Blocklist {
    labels: Vec<Node>
}

impl Blocklist {
    const ROOT_ELEMENT: usize = 0;

    fn init() -> Self {
        let mut bl = Self::new();

        file::get_elements().into_iter().for_each(|n| {
            let mut labels = n.labels.as_slices().0.iter().cloned().collect::<Vec<Label>>();
            let mut wildcard = false;
            if let Some(l) = labels.get(0) {
                if l.as_str() == "*" {
                    wildcard = true;
                }
            }
            if wildcard {
                labels.remove(0);
            }
            bl.add_element(labels, wildcard)
        });

        bl        
    }

    pub fn new() -> Self {
        let mut labels = Vec::with_capacity(32);
        labels.push(Node::new(false, false));
        Self {
            labels
        }
    }

    pub fn add_element(&mut self, mut name: Vec<Label>, wildcard: bool) {
        let mut element = Self::ROOT_ELEMENT;
        while name.len() > 0 {
            let label = name.pop().unwrap();
            log::debug!("Inserting {} in the blocklist, wildcard={}, blocked={}", label.as_str(), wildcard && name.len() == 0, name.len() == 0);
            match self.labels[element].children.get(label.as_str()) {
                Some(c) => {
                    element = *c;
                    if name.len() == 0 {
                        self.labels[element].blocked = true;
                    }
                },
                None => {
                    let element_count = self.labels.len();
                    self.labels.push(Node::new(name.len() == 0, name.len() == 0 && wildcard));
                    self.labels[element].children.insert(label.value.clone(), element_count);
                    element = element_count;
                }
            }
            // check si le label existe
            // si oui, va au suivant
            // si non, ajoute le et va au suivant.
        }
    }

}

pub fn contains(mut name: &[Label]) -> bool {
    let mut element = Blocklist::ROOT_ELEMENT;
    while !BLOCKLIST.labels[element].is_leaf() {
        log::debug!("Checking if blocklist contains {}", name[name.len() - 1].as_str());
        log::debug!("Exploring element with wildcard={}", BLOCKLIST.labels[element].wildcard);
        if BLOCKLIST.labels[element].wildcard {
            return true;
        }
        match BLOCKLIST.labels[element].children.get(name[name.len() - 1].as_str()) {
            Some(c) => {
                // TODO: Add better formulation
                element = *c;
                if name.len() == 1 {
                    return BLOCKLIST.labels[element].blocked;
                } else {
                    name = &name[0..name.len() - 1];
                }
            },
            None => {
                return false;
            }
        }
    }

    BLOCKLIST.labels[element].wildcard
}

struct Node {
    children: HashMap<String, usize, Hasher::Hash64>,
    wildcard: bool,
    blocked: bool,
}

impl Node {
    fn new(blocked: bool, wildcard: bool) -> Self {
        Self {
            children: HashMap::with_hasher(Hasher::Hash64),
            wildcard,
            blocked
        }
    }

    fn is_leaf(&self) -> bool {
        self.children.len() == 0
    }

}
