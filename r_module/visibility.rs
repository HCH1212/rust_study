mod my_mod {
    // 模块内私有函数
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 公有
    pub fn function() {
        println!("called `my_mod::function()`");
        private_function(); // mod 内部可以调用私有函数
    }

    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }
    }
}

fn main() {
    my_mod::function();
    my_mod::nested::function();
    // my_mod::private_function();
}
