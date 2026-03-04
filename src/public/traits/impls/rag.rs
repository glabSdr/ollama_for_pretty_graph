use pretty_graph::Node;
use crate::OllamaForPrettyGraph;
use crate::public::Rag;

impl Rag for Node {
    fn set_header(&self, header: &str) {
        self.set("header", header);
    }

    fn set_instruction(&self, instruction: &str) {
        self.set("instruction", instruction);
    }
    fn execute_with_context_and_query(&self, context: &str, query: &str) {
        let mut prompt = self.get("header").unwrap_or(String::new());

        prompt.push_str("CONTEXT: \n");
        prompt.push_str(context);
        prompt.push('\n');
        prompt.push('\n');

        prompt.push_str("QUERY: \n");
        prompt.push_str(query);
        prompt.push('\n');
        prompt.push('\n');

        let instruction = self.get("instruction");
        if instruction.is_some() {
            prompt.push_str(&instruction.unwrap());
        }

        self.execute_blocking(&prompt)
    }
}

#[cfg(test)]
mod tests {
    use crate::OLLAMA_DEFAULT_URL;
    use super::*;
    #[test]
    fn test_set_header() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model");
        node.set_header("Some header");
    }

    #[test]
    fn test_set_instruction() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "model");
        node.set_instruction("Some instruction");
    }

    #[test]
    fn test_execute_with_context_and_query() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.execute_with_context_and_query("Context (RAG)", "User query");

        node.set_instruction("Some instruction");
        node.execute_with_context_and_query("Context (RAG)", "User query");
    }
}