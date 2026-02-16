use pretty_graph::Node;
use crate::private::constants::{BASE_URL_K, MODEL_K, SYSTEM_K};
use crate::private::traits::traits::GetUsefulFields;

impl GetUsefulFields for Node {
    fn get_useful_fields(&self) -> (String, String, Option<String>) {
        let base_url = self.get(BASE_URL_K).unwrap();
        let model = self.get(MODEL_K).expect("Field model required for execution");
        let system = self.get(SYSTEM_K);

        (base_url, model, system)
    }
}