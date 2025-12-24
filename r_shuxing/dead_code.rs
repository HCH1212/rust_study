fn used_function() {}

// `#[allow(dead_code)]` 属性可以禁用 `dead_code` lint
// 没有使用也不会警告
#[allow(dead_code)]
fn unused_function() {}

fn noisy_unused_function() {}

fn main() {
    used_function();
}
