
pub fn Injector() {
  
}

pub trait Factory {
  fn get() -> Box<Self>;
}