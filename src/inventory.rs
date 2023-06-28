use std::io::{self, BufRead};

struct Node{
    
}

impl Node{
    fn new() -> Self{
        Self{
            
        }
    }
    fn render(&self){
        println!("--------------------");
        println!("       AK-47");
        println!("--------------------");
    }
}

pub struct InventoryManager{
    nodes: Vec<Node>,
}

impl InventoryManager{
    pub fn new() -> Self{
        Self{
            nodes: Vec::new(),
        }
    }
    
    pub fn add_node(&mut self){
        self.nodes.push(Node::new());
    }
    
    pub fn render(&mut self){
        for node in self.nodes.iter(){
            node.render();
        }
    }
    
    pub fn update(&self) -> bool{
        let stdin = io::stdin();
        let input = stdin.lock().lines().next().unwrap().unwrap();        
        true
    }
}

