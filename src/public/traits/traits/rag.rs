
/// Trait Rag provides instrument to build structured prompt with structure like this:
/// {header} - your user prompt upper part.
/// CONTEXT:
/// {context} - context from external src (formated as a text embedding)
/// QUERY:
/// {query} - user input
/// {instruction} - some extra instruction before start generation
pub trait Rag {
    /// Set prompt header. Not the same as system prompt!
    /// ```rust
    ///use pretty_graph::Node;
    ///use ollama_for_pretty_graph::*;
    ///
    ///let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model");
    ///node.set_header("Some header");
    /// ```
    fn set_header(&self, header: &str);


    /// Set prompt instruction.
    /// ```rust
    ///use pretty_graph::Node;
    ///use ollama_for_pretty_graph::*;
    ///
    ///let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model");
    ///node.set_instruction("Some instruction");
    /// ```
    fn set_instruction(&self, instruction: &str);


    /// Set prompt instruction.
    /// ```rust
    ///use pretty_graph::Node;
    ///use ollama_for_pretty_graph::*;
    ///
    ///let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model");
    /// // ...
    ///node.execute_with_context_and_query("Context (RAG)", "User query");
    /// ```
    fn execute_with_context_and_query(&self, context: &str, query: &str);
}