struct Siswa{
  name: String,
  age: u32,
  mark: u32,
}
impl Siswa{
  fn newbie(name: &str, age: u32, mark: u32) -> Siswa {
    Siswa {
      name: String::from(name),
      age: age,
      mark: mark,
    }
  }
  fn passed(&self) -> &str {
    if self.mark >= 75 {
      "lulus"
    }else {
      "tidak lulus"
    }
  }
  fn prints(&self){
    println!("name: {}", self.name);
    println!("age: {}", self.age);
    println!("mark: {}", self.mark);
    println!("status: {}", self.passed());
  }
}
fn main() {
  let siswa: Siswa;
  siswa = Siswa::newbie("izra", 18, 75);
  Siswa::prints(&siswa);
}
