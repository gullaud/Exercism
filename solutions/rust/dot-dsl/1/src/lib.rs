pub mod graph {
    use std::collections::HashMap;
    use graph_items::edge::Edge;
    use graph_items::node::Node;

    pub struct Graph {
        pub nodes : Vec<Node>,
        pub edges : Vec<Edge>,
        pub attrs : HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes : Vec::new(),
                edges : Vec::new(),
                attrs : HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, nodes : &Vec::<Node>) -> Self {
            for n in nodes {
                self.nodes.push(n.clone());
            }
            self
        }
        pub fn with_edges(mut self, edges : &Vec::<Edge>) -> Self {
            self.edges.extend(edges.iter().cloned());
            self
        }
        pub fn with_attrs(mut self, edges : &[(&str, &str)]) -> Self {
            for (k,v) in edges {
                self.attrs.insert(k.to_string(),v.to_string());
            }
            self
        }

        pub fn node(&self, name:&str) -> Option<Node> {
            if let Some(n) = self.nodes.iter().position(|a| a.name == name) {
                return Some(self.nodes[n].clone());
            }
            return None;
        }
    }
    pub mod graph_items {
        pub mod edge {
            use std::collections::HashMap;
            #[derive(Clone, PartialEq, Debug)]
            pub struct Edge {
                node_a : String,
                node_b : String,
                attrs : HashMap<String,String>
            }
            impl Edge {
                pub fn new(node_a: &str, node_b: &str) -> Self {
                    Self {
                        node_a : node_a.to_string(), 
                        node_b : node_b.to_string(),
                        attrs : HashMap::new(),
                    }
                }
                pub fn attr(&self, name: &str) -> Option<&str> {
                    if let Some (s) = self.attrs.get(&name.to_string()) {
                        return Some(&s);
                    }
                    None
                }
                pub fn with_attrs(mut self, edges : &[(&str, &str)]) -> Self {
                    for (k,v) in edges {
                        self.attrs.insert(k.to_string(),v.to_string());
                     }
                     self
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            #[derive(Clone, PartialEq, Debug)]
            pub struct Node {
                pub name : String,
                pub attrs : HashMap<String, String>
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Self { name : name.to_string(), attrs : HashMap::new() }
                }
                pub fn with_attrs(mut self, attrs : &[(&str, &str)]) -> Self {
                   for (a,b) in attrs {
                        self.attrs.insert(a.to_string(),b.to_string());
                    }
                    self
                }
                pub fn attr(&self, name: &str) -> Option<&str> {
                    if let Some (s) = self.attrs.get(&name.to_string()) {
                        return Some(&s);
                    }
                    None
                }
            }
        }
    }
}
