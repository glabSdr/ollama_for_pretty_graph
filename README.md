# Ollama 🦙 for pretty_graph 🎀

This crate provide basic access to ollama api for systems based on pretty_graph

*This crate does not imply full integration of the Ollama API, its purpose is to extend the capabilities of the Node entity from the pretty_graph crate in the field of working with Ollama!* 

## Short intro
```rust
use pretty_graph::Node;
use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};

fn main() {
    let agent = Node::new_ollama_node(OLLAMA_DEFAULT_URL,"qwen2.5:7b");
    agent.execute_blocking("Hello!");

    println!("{}", agent.resp());
}
```

## Take in mind
This module reserves some node payload keys. Don't use them manually in Node(s) you're going to use with this trait as it can brake your program logic.

List:
- model
- base_url
- system
- last_raw_resp
