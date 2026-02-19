use crate::Config;

pub trait OllamaForPrettyGraph {

    /// Create new node with ollama support
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// ```
    fn new_ollama_node(base_url: &str, model: &str) -> Self;




    /// Set system prompt.
    /// System prompt will be sent with any user prompt which will be sent using this node
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.set_system_prompt("some system prompt");
    /// ```
    fn set_system_prompt(&self, sp: &str);




    /// Blocking execution with custom input
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.execute_blocking("Hello!");
    /// ```
    fn execute_blocking(&self, input: &str);


    /// Blocking execution with custom input and config
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.execute_blocking_with_config("Hello!", Config::Default());
    /// ```
    ///
    fn execute_blocking_with_config(&self, prompt: &str, config: Config);




    /// Blocking execution with custom input and format
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.execute_blocking_with_custom_format("Hello!", "json");
    /// println!({}, node.resp())
    /// ```
    fn execute_blocking_with_custom_format(&self, input: &str, format: &str);



    /// Blocking execution with custom input and format and config
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.execute_blocking_with_custom_format_and_config("Hello!", "json", Config::Default());
    /// ```
    ///
    fn execute_blocking_with_custom_format_and_config(&self, prompt: &str, format: &str, config: Config);




    /// Get a string response
    /// Example:
    /// ```rust
    /// let node = Node::new_ollama_node();
    /// node.execute_blocking("Hello!");
    /// println!({}, node.resp())
    /// ```
    fn resp(&self) -> String;
}