use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub enum NodeType {
    Element(Element),
    Content(String),
}

#[derive(Debug)]
pub struct Element {
    pub tagname: String,
    pub attributes: AttrMap
}

impl Element {
    pub fn get_attribute(&self, attr: String) -> Option<&String> {
        self.attributes.get(&attr)
    }
}

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub nodetype: NodeType,
}

impl Node {
    pub fn text(text: String) -> Node {
        Node {
            children: vec![],
            nodetype: NodeType::Content(text),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children: children,
            nodetype: NodeType::Element(Element {
                 tagname: name,
                 attributes: attrs,
            })
        }
    }
}
