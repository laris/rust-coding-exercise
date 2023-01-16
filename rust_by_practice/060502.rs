struct Unit;
trait SomeTrait {}
// 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
// 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
impl SomeTrait for Unit {}

fn main() {
    let u = Unit;
    do_something_with_unit(u);
}

fn do_something_with_unit(u: Unit) { }
