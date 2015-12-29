use std::fmt;
use std::collections::HashMap;

pub type AttrMap = HashMap<String, String>;

#[derive(PartialEq, Debug)]
pub enum NodeType {
    Element(Element),
    Content(String),
    Comment,
}

#[derive(PartialEq, Debug)]
pub struct Element {
    pub tagname: String,
    pub attributes: AttrMap,
    pub void: bool,
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

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.nodetype {
            NodeType::Content(ref text) => {
                write!(f, "{}\n", text)
            },
            NodeType::Element(ref elem) => {
                write!(f, "<{}", elem.tagname).unwrap();
                //attributes
                for attr in &elem.attributes {
                    let (name, value) = attr;
                    write!(f, " {}", name).unwrap();
                    if value.len() > 0 {
                         write!(f, "=\"{}\"", value).unwrap();
                    }
                }
                if elem.void {
                    return write!(f, " />\n");
                }
                write!(f, ">\n").unwrap();
                for child in &self.children {
                    write!(f, "{}", child).unwrap();
                }
                write!(f, "</{}>\n", elem.tagname)
            },
            _ => {
                write!(f, "{:?}", self.nodetype)
            },
        }
    }
}

impl Node {
    pub fn text(text: String) -> Node {
        Node {
            children: vec![],
            nodetype: NodeType::Content(text),
        }
    }

    pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>, void: bool) -> Node {
        Node {
            children: children,
            nodetype: NodeType::Element(Element {
                 tagname: name,
                 attributes: attrs,
                 void: void
            })
        }
    }

    pub fn comment() -> Node {
        Node {
            children: vec![],
            nodetype: NodeType::Comment,
        }
    }
}
