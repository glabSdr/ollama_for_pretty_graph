use crate::Config;

pub trait OllamaForPrettyGraph {

    /// Create new node with ollama support
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// ```
    fn new_ollama_node(base_url: &str, model: &str) -> Self;




    /// Set system prompt.
    /// System prompt will be sent with any user prompt which will be sent using this node
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.set_system_prompt("some system prompt");
    /// ```
    fn set_system_prompt(&self, sp: &str);




    /// Blocking execution with custom input
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.execute_blocking("Hello!");
    /// ```
    fn execute_blocking(&self, input: &str);


    /// Blocking execution with custom input and config
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL, Config};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.execute_blocking_with_config("Hello!", Config::default());
    /// ```
    ///
    fn execute_blocking_with_config(&self, prompt: &str, config: Config);


    /// Blocking execution with custom input and format
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.execute_blocking_with_custom_format("Hello!", "json");
    /// println!("{}", node.resp())
    /// ```
    fn execute_blocking_with_custom_format(&self, input: &str, format: &str);



    /// Blocking execution with custom input and format and config
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL, Config};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.execute_blocking_with_custom_format_and_config("Hello!", "json", Config::default());
    /// ```
    ///
    fn execute_blocking_with_custom_format_and_config(&self, prompt: &str, format: &str, config: Config);




    /// Get a string response
    /// Example:
    /// ```rust
    /// use pretty_graph::Node;
    /// use ollama_for_pretty_graph::{OllamaForPrettyGraph, OLLAMA_DEFAULT_URL};
    ///
    /// let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model_name");
    /// node.execute_blocking("Hello!");
    /// println!("{}", node.resp())
    /// ```
    fn resp(&self) -> String;
}