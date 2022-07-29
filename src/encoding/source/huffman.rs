use crate::encoding::source::SourceCoding;
use crate::Message;
use std::collections::{HashMap, VecDeque};

pub struct HuffmanCoding {
    _code: HashMap<u8, String>,
}

impl SourceCoding for HuffmanCoding {
    fn generate_code(msg: &Message) -> HashMap<u8, String> {
        let freq_map = HuffmanCoding::generate_freq_map(msg);
        let tree_node = make_tree(&freq_map);
        let huffman_map = make_huffman_map(&tree_node.0, None);
        return huffman_map;
    }

    fn encode(&self, msg: &Message) -> Message {
        // TODO: Implement this
        return Message::new();
    }

    fn decode(&self, msg: &Message) -> Message {
        // TODO: Implement this
        return Message::new();
    }
}

impl HuffmanCoding {
    pub fn new() -> HuffmanCoding {
        return HuffmanCoding {
            _code: HuffmanCoding::empty_char_map(),
        };
    }

    pub fn code(mut self, new_code: HashMap<u8, String>) -> HuffmanCoding {
        self._code = new_code;
        return self;
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|===================================| Data structures |=======================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
struct HuffmanTreeNode {
    pub _data: Option<u8>,
    pub _left: Option<Box<HuffmanTreeNode>>,
    pub _right: Option<Box<HuffmanTreeNode>>,
}

impl HuffmanTreeNode {
    pub fn new() -> HuffmanTreeNode {
        return HuffmanTreeNode {
            _data: None,
            _left: None,
            _right: None,
        };
    }

    pub fn data(mut self, val: u8) -> HuffmanTreeNode {
        self._data = Some(val);
        return self;
    }

    pub fn left(mut self, val: Box<HuffmanTreeNode>) -> HuffmanTreeNode {
        self._left = Some(val);
        return self;
    }

    pub fn right(mut self, val: Box<HuffmanTreeNode>) -> HuffmanTreeNode {
        self._right = Some(val);
        return self;
    }

    pub fn get_children(&self) -> (&Box<HuffmanTreeNode>, &Box<HuffmanTreeNode>) {
        return (self._left.as_ref().unwrap(), self._right.as_ref().unwrap());
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////
//|=================================| Auxiliary functions |=====================================|//
///////////////////////////////////////////////////////////////////////////////////////////////////

fn make_tree(freq_map: &HashMap<u8, u32>) -> (HuffmanTreeNode, u32) {
    let mut result = VecDeque::<(HuffmanTreeNode, u32)>::new();
    for it in freq_map {
        result.push_back((HuffmanTreeNode::new().data(*it.0), *it.1));
    }
    result.make_contiguous().sort_by(|a, b| a.1.cmp(&b.1));

    while result.len() > 1 {
        let c1 = result.pop_front().unwrap();
        let c2 = result.pop_front().unwrap();
        let node = HuffmanTreeNode::new()
            .left(Box::new(c1.0))
            .right(Box::new(c2.0));

        result.push_back((node, c1.1 + c2.1));
        result.make_contiguous().sort_by(|a, b| a.1.cmp(&b.1));
    }

    return result[0].clone();
}

fn make_huffman_map(tree_root: &HuffmanTreeNode, binary_code: Option<String>) -> HashMap<u8, String> {
    match tree_root._data {
        Some(val) => {
            let mut output = HashMap::new();
            output.insert(val, match binary_code {
                Some(code) => code,
                None => "".to_string(),
            });
            output
        },
        None => {
            let children = tree_root.get_children();
            let mut left_map = make_huffman_map(&(**children.0).clone(), Some(match binary_code.clone() {
                Some(code) => code,
                None => "".to_string(),
            } + "0"));
            let right_map = make_huffman_map(&(**children.1).clone(), Some(match binary_code.clone() {
                Some(code) => code,
                None => "".to_string(),
            } + "1"));
            left_map.extend(right_map);
            left_map
        }
    }
}
