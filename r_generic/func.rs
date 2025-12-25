struct A;          // 具体类型 `A` 单元结构体
struct S(A);       // 具体类型 `S` 元组结构体
struct SGen<T>(T); // 泛型类型 `SGen` 元组结构体

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));
    generic::<char>(SGen('a')); // 显式地指定类型参数 `char`
    generic(SGen('c')); // 隐式地指定类型参数 `char`
}
