pub trait GetUsefulFields {
    fn get_useful_fields(&self) -> (String, String, Option<String>);
}