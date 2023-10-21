use std::fmt::Debug;

use super::dto::{label::Label, name::Name};

#[derive(Default, Clone)]
struct TreeElement {
    referenced_label: ReferencedLabel,
    children: Vec<usize>,
}

impl Debug for TreeElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, referenced: {:?}", self.referenced_label, self.children)
    }
}


pub struct LabelTree {
    elements: Vec<TreeElement>
}

impl Debug for LabelTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}

impl LabelTree {
    const ROOT_NODE: usize = 0;

    /// Inserts a list of referenced labels inside the reference tree.
    ///
    /// # Arguments
    ///
    /// * `referenced_labels` - A vector of ReferencedLabel that is in reversed label order
    ///     i.e. www.antoinec.dev would have the labels ordered as [dev, antoinec, www].
    pub fn insert(&mut self, referenced_labels: Vec<ReferencedLabel>) {
        let mut node: usize = Self::ROOT_NODE;
        let mut current_index: usize = 0;
        
        if referenced_labels.len() == 0 {
            return;
        }

        loop {
            match self.find_child(node, &referenced_labels[current_index].label) {
                Some(c) => {
                    node = c;
                    current_index += 1;
                },
                None => {
                    for rl in referenced_labels[current_index..].iter() {
                        self.elements.push(
                            TreeElement {
                                referenced_label: rl.clone(),
                                children: Vec::new(),
                            }
                        );

                        let current_length = self.elements.len();
                        self.elements[node].children.push(current_length - 1);
                        node = self.elements.len() - 1;
                    }

                    break;
                }
            }

            if current_index >= referenced_labels.len() {
                break;
            }
        }
    }

    #[inline]
    fn find_child(&self, parent_index: usize, child: &Label) -> Option<usize> {
        self.elements[parent_index].children.iter().find(|i| {self.elements[**i].referenced_label.label == *child}).copied()
    }

    pub fn find_best_reference(&self, name: &Name) -> CompressionReference {
        let mut best_reference = CompressionReference {
            index: 0,
            position: 0,
        };
        let mut node = Self::ROOT_NODE;

        if name.labels.len() == 0 {
            return best_reference;
        }

        loop {
            match self.find_child(node, &name.labels[name.labels.len() - 1 - best_reference.index]) {
                Some(e) => {
                    best_reference.position = self.elements[e].referenced_label.position;
                    best_reference.index += 1;
                    node = e;
                },
                None => return best_reference,
            }

            if best_reference.index == name.labels.len() {
                break;
            }
        }
        best_reference
    }
}

impl Default for LabelTree {
    fn default() -> Self {
        Self {
            elements: vec![
                TreeElement {
                    referenced_label: ReferencedLabel {
                        label: "".into(),
                        position: 0,
                    },
                    children: vec![],
                }
            ],
        }
    }
}

pub struct CompressionReference {
    /// The index of the label that is being referenced in reversed order.
    /// i.e. for www.example.google.com, with an index of 1,
    /// we would have .com that is already referenced.
    pub index: usize,

    /// The position of the reference so that the pointer can be generated.
    pub position: u16,
}

impl CompressionReference {
    pub fn is_valid(&self) -> bool {
        self.index > 0
    }
}

#[derive(Clone, Default)]
pub struct ReferencedLabel {
    label: Label,
    position: u16,
}

impl ReferencedLabel {
    pub fn new(label: Label, position: u16) -> Self { Self { label, position } }
}

impl Debug for ReferencedLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Label {} at {}", self.label, self.position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_insertion() {
        let mut lt: LabelTree = LabelTree::default();

        let referenced_labels = vec![
            ReferencedLabel::new(Label::from("dev"), 13),
            ReferencedLabel::new(Label::from("antoinec"), 4),
            ReferencedLabel::new(Label::from("www"), 0),
        ];
        lt.insert(referenced_labels);
        let referenced_labels = vec![
            ReferencedLabel::new(Label::from("dev"), 13),
            ReferencedLabel::new(Label::from("charbonneau"), 4),
            ReferencedLabel::new(Label::from("www"), 0),
        ];
        lt.insert(referenced_labels);

        assert_eq!(&lt.elements[4].referenced_label.label.to_string(), "charbonneau");
        assert!(lt.elements[4].children.contains(&5));
        assert!(lt.elements[1].children.contains(&2));
        assert!(lt.elements[1].children.contains(&4));
    }

    #[test]
    fn test_best_reference() {
        let mut lt: LabelTree = LabelTree::default();

        let referenced_labels = vec![
            ReferencedLabel::new(Label::from("dev"), 13),
            ReferencedLabel::new(Label::from("antoinec"), 4),
            ReferencedLabel::new(Label::from("www"), 0),
        ];
        lt.insert(referenced_labels);
        let referenced_labels = vec![
            ReferencedLabel::new(Label::from("dev"), 13),
            ReferencedLabel::new(Label::from("charbonneau"), 4),
            ReferencedLabel::new(Label::from("www"), 0),
        ];
        lt.insert(referenced_labels);

        let reference = lt.find_best_reference(&Name::from("mail.antoinec.dev"));

        assert_eq!(reference.index, 2);
        assert_eq!(reference.position, 4);
    }
}
