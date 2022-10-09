trait Person {
  fn name(&self) -> String;
}

// Person is a supertrait of Student.
// Implementing Student requires you to also impl Person.
trait Student: Person {
  fn university(&self) -> String;
}

trait Programmer {
  fn fav_language(&self) -> String;
}

// CompSciStudent (computer science student) is a subtrait of both Programmer 
// and Student. Implementing CompSciStudent requires you to impl both supertraits.
trait CompSciStudent: Programmer + Student {
  fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
  format!(
      "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
      student.name(),
      student.university(),
      student.fav_language(),
      student.git_username()
  )
}


struct AStu {
  sname: &'static str,
  suniv: &'static str,
  slang: &'static str,
  suname: &'static str,
}

impl Person for AStu {
  fn name(&self) -> String { (&self.sname).to_string() }
}
impl Student for AStu {
  fn university(&self) -> String { (&self.suniv).to_string() }
}
impl Programmer for AStu {
  fn fav_language(&self) -> String { (&self.slang).to_string() }
}
impl ComSciStudent for AStu {
  fn git_username(&self) -> String { (&self.suname).to_string() }
}

fn main() {
  let s = AStu { sname: "myname", suniv: "myuniv", slang: "myl", suname: "myuname"};
  println!("{}", com_sci_student_greeting(&s));

}
