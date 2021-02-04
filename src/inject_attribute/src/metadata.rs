use std::collections::HashMap;

pub enum ObjetType {
  IsSingleton,
  IsFactory,
}

pub struct StructMeta {
  object_type: ObjetType,
  tag: String,
  identifier: String,
  dependencies: Vec<String>,
  cache: Option<String>
}

// pub struct SomeMap {
//   metadata: Box<HashMap<String, String>>
// }

// const some_map_intance: SomeMap = SomeMap{
//   metadata: Box::new(HashMap::new())
// };

// impl SomeMap {

//   pub fn get_instance() -> &'static SomeMap {
//     &some_map_intance
//   }
// }

// const fn create_map() -> HashMap<String, String> {
//   HashMap::new()
// }
// const Metadata: HashMap<String, String> = HashMap::new();