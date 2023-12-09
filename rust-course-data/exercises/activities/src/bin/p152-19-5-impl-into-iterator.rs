#[derive(Debug)]
struct Friends {
    names: Vec<String>,
}

impl IntoIterator for Friends {
    type Item = String;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.into_iter()
    }
}

impl<'a> IntoIterator for &'a Friends {
    type Item = &'a String;
    type IntoIter = std::slice::Iter<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter()
    }
}

impl<'a> IntoIterator for &'a mut Friends {
    type Item = &'a mut String;
    type IntoIter = std::slice::IterMut<'a, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.names.iter_mut()
    }
}

impl Friends {
    fn iter(&self) -> std::slice::Iter<'_, String> {
        self.into_iter()
    }
    fn iter_mut(&mut self) -> std::slice::IterMut<'_, String> {
        self.into_iter()
    }
}
fn main() {
    let mut friends = Friends { names: vec![] };
    friends.names.push("Jimmy".into());
    friends.names.push("Tommy".into());
    friends.names.push("Annie".into());
    friends.names.push("Robert".into());

    for f in &friends {
        println!("{f:?}");
    }
    //for f in friends {
    //    println!("{f:?}");
    //}

    for f in &friends {
        println!("{f:?}");
    }

    for f in &mut friends {
        *f = "Frank".into();
        println!("{f:?}");
    }

    let total = friends.iter().count();
    println!("{total}");
}
