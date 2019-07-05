/*
 * Rustのスマートポインタ（Rc<T>）。
 * CreatedAt: 2019-07-05
 */
fn main() {
    let x = 5;
    let y = &mut x; // error[E0596]: cannot borrow immutable local variable `x` as mutable
}
