use pretty_graph::Node;
use crate::private::constants::{BASE_URL_K, LAST_RAW_RESP, MODEL_K, SYSTEM_K};
use crate::{Config, OllamaForPrettyGraph};
use crate::private::{execute_blocking, GetUsefulFields};


impl OllamaForPrettyGraph for Node {
    fn new_ollama_node(base_url: &str, model: &str) -> Self {
        let node = Self::new();
        node.set(MODEL_K, model);
        node.set(BASE_URL_K, base_url);
        node
    }

    fn set_system_prompt(&self, system: &str) {
        self.set(SYSTEM_K, system);
    }
    fn execute_blocking(&self, prompt: &str)  {
        let (base_url, model, system) = self.get_useful_fields();
        let resp = execute_blocking(&base_url, model, system, prompt.to_string(), None, None);
        self.set(LAST_RAW_RESP, &resp);
    }

    fn execute_blocking_with_config(&self, prompt: &str, config: Config)  {
        let (base_url, model, system) = self.get_useful_fields();
        let resp = execute_blocking(&base_url, model, system, prompt.to_string(), None, Some(config));
        self.set(LAST_RAW_RESP, &resp);
    }

    fn execute_blocking_with_custom_format(&self, prompt: &str, format: &str)  {
        let (base_url, model, system) = self.get_useful_fields();
        let resp = execute_blocking(&base_url, model, system, prompt.to_string(), Some(format.to_string()), None);
        self.set(LAST_RAW_RESP, &resp);
    }

    fn execute_blocking_with_custom_format_and_config(&self, prompt: &str, format: &str, config: Config)  {
        let (base_url, model, system) = self.get_useful_fields();
        let resp = execute_blocking(&base_url, model, system, prompt.to_string(), Some(format.to_string()), Some(config));
        self.set(LAST_RAW_RESP, &resp);
    }

    fn resp(&self) -> String {
        self.get(LAST_RAW_RESP).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::OLLAMA_DEFAULT_URL;
    use super::*;

    #[test]
    fn test_new_ollama_node() {
        let _ = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");

    }

    #[test]
    fn test_set_system_prompt() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.set_system_prompt("test system prompt");
    }

    #[test]
    fn test_execute_blocking() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.execute_blocking("test input");
    }

    #[test]
    fn test_execute_blocking_with_system_set() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.set_system_prompt("test system prompt");
        node.execute_blocking("test input");
    }

    #[test]
    fn test_execute_blocking_with_config() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.execute_blocking_with_config("test input", Config::default());
    }


    #[test]
    fn test_empty_execute_blocking_with_custom_format() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.execute_blocking_with_custom_format("test input", "format");
    }

    #[test]
    fn execute_blocking_with_custom_format_and_config() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.execute_blocking_with_custom_format_and_config("test input", "json",  Config::default());
    }

    #[test]
    #[should_panic]
    fn test_execute_blocking_execution() {
        let node = Node::new();
        node.execute_blocking("test input");
    }

    #[test]
    fn test_resp() {
        let node = Node::new_ollama_node(OLLAMA_DEFAULT_URL, "test");
        node.set(LAST_RAW_RESP, "ex");
        assert_eq!(node.resp(), "ex");
    }
}

