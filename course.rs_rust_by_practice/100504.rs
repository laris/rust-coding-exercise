trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}

trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. \nMy favorite language is {}. \nMy Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username(),
    )
}

struct CSStudent {
    name: String,
    university: String,
    fav_language: String,
    git_username: String,
}

impl Person for CSStudent { fn name(&self) -> String { self.name.clone() }}
impl Student for CSStudent { fn university(&self) -> String { self.university.clone() }}
impl Programmer for CSStudent { fn fav_language(&self) -> String { self.fav_language.clone() }}
impl CompSciStudent for CSStudent { fn git_username(&self) -> String { self.git_username.clone() }}

fn main() {
    let student = CSStudent {
        name: "Sunfei".to_string(),
        university: "PKU".to_string(),
        fav_language: "Rust".to_string(),
        git_username: "sunface".to_string(),
    };

    println!("{}", comp_sci_student_greeting(&student));
}
