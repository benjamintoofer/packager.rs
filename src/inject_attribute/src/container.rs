use crate::metadata;

pub struct Container {
    
}

impl Container {

  pub fn bind(&self) {
    // metadata::Metadata
  }

  // Should give T as a trait. Pass some identifier with an optional tag?
  pub fn get<T>(&self, iden: String) {

  }

  fn resolve(&self) {

  }
}

trait Factory {
  fn get() -> Box<Self>;
}

struct Tanya {

}

impl Factory for Tanya {
  fn get() -> Box<Tanya> {
    Box::new(Tanya{})
  }
}

impl Tanya {
  pub fn tanya_stuff(&self) {

  }
}

trait FakeTrait {
  fn do_fake_stuff(&self);
}

trait IWhatever {
  fn some_whatever_stuff(&self);
}
struct BenDepend {

}

impl FakeTrait for BenDepend {
  fn do_fake_stuff(&self) {
      
  }
}

impl Factory for BenDepend {
  fn get() -> Box<BenDepend> {
    Box::new(BenDepend{})
  }
}

struct Ben {
  depen: Box<dyn FakeTrait>
}

impl Factory for Ben {
  fn get() -> Box<Ben> {
    let depend = call_something::<BenDepend>();
    Box::new(Ben{depen: depend})
  }
}

impl Ben {
  pub fn ben_stuff(&self) {

  }
}

fn call_something<T>() -> Box<T>
where
T: Factory {
  T::get()
}

fn getter<T>() -> Box<T>
where
T: Factory {
  T::get()
}

fn temp() {
  let b = Ben::get();
  let t = Tanya::get();
  t.tanya_stuff();
  b.ben_stuff();
  let tanya_2 = call_something::<Tanya>();
  let ben_2 =  call_something::<Ben>();
  ben_2.ben_stuff();
  ben_2.depen.do_fake_stuff()
}
