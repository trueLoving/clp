#[derive(Debug)]
pub struct Core {
  u_id: i32,
  flags: String,
  icons: String,
  colors: String,
  sorter: String,
}

impl Core {
  pub fn create() -> Core {
    return Core {
      u_id: 32,
      flags: "flags".to_uppercase(),
      icons: "icons".to_uppercase(),
      colors: "colors".to_uppercase(),
      sorter: "sorter".to_uppercase(),
    };
  }
  pub fn run(self) {
    println!("core start run");
  }
}
