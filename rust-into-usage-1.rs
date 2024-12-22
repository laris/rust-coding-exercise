#[derive(Default)]
pub struct Collector {
    infos: Vec<(String, String)>,
}

impl<A, B> Extend<(A, B)> for Collector
where
    A: Into<String>,
    B: Into<String>,
{
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (A, B)>,
    {
        self.infos.extend(iter.into_iter().map(|(a, b)| (a.into(), b.into())));
    }
}

pub fn main() {
    let flag = 1; //模拟不同条件
    
    let mut collector = Collector::default();
    collector.extend([("k_base", "v_base")]);

    match flag {
        0 => collector.extend([("k_0", "v_0")]),
        1 => collector.extend([("k_1", "v_1"), ("k_11", "v_11")]),
        _ => {}
    };
}

