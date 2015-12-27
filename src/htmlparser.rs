use dom;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = vec!();

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }
            nodes.push(self.parse_node());
        }
        nodes
    }

    fn parse_node(&mut self) -> dom::Node {
        match self.next_char(false) {
            '<' => self.parse_element(),
            _   => self.parse_text(),
        }
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::Node::text(self.consume_while(|c| c!= '<'))
    }

    fn parse_element(&mut self) -> dom::Node {

        assert!(self.next_char(true) == '<');
        let tagname = self.parse_tagname();
        let attrs   = self.parse_attributes();
        assert!(self.next_char(true) == '>');

        //child nodes
        let children = self.parse_nodes();

        assert!(self.next_char(true) == '<');
        assert!(self.next_char(true) == '/');
        assert!(self.parse_tagname() == tagname);
        assert!(self.next_char(true) == '>');

        dom::Node::elem(tagname, attrs, children)
    }

    fn parse_tagname(&mut self) -> String {
        self.consume_while(|c| match c {
            'a'...'z' | 'A'...'Z' | '0'...'9'   => true,
            _                                   => false,
        })
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = dom::AttrMap::new();

        loop {
            self.consume_whitespace();
            if self.next_char(false) == '>' {
                break;
            }
            let (name, value) = self.parse_attribute();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_attribute(&mut self) -> (String, String) {
         let name = self.parse_tagname();
         assert!(self.next_char(true) == '=');
         let value = self.parse_string();

         (name, value)
    }

    fn parse_string(&mut self) -> String {
         let quote = self.next_char(true);
         assert!(quote =='"' || quote == '\'');
         let string = self.consume_while(|c| c != quote);
         assert!(self.next_char(true) == quote);

         string
    }

    fn next_char(&mut self, consume: bool) -> char {
        let mut iter = self.input[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        if consume {
            self.pos += next_pos;
        }
        return cur_char;
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn consume_while<F: Fn(char) -> bool>(&mut self, test: F) -> String {
        let mut res = String::new();
        while !self.eof() && test(self.next_char(false)) {
            res.push(self.next_char(true));
        }
        res
    }
    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }
}

pub fn parse(input: String) -> dom::Node {
    let mut dom_object = Parser { pos: 0, input: input }.parse_nodes();

    if dom_object.len() == 1 {
         dom_object.swap_remove(0)
    } else {
        dom::Node::elem("html".to_owned(), dom::AttrMap::new(), dom_object)
    }
}
