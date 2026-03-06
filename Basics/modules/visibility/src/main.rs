use crate::struct_mod::{OpenBox, CloseBox};
use crate::custom_mod::public_func as other_func;

mod struct_mod {
    pub struct OpenBox {
        pub content: String,
    }

    pub struct CloseBox {
        content: String,
    }

    impl CloseBox {
        pub fn new(content: String) -> CloseBox {
            CloseBox { content: content }
        }
    }
}

mod custom_mod {
    fn private_func() {
        println!("Called custom_mod::private_func");
    }

    pub fn public_func() {
        println!("Called custom_mod::public_func");
    }

    pub fn indirect_func() {
        println!("Called custom_mod::indirect_func");
        private_func();
    }

    pub mod nested {
        pub fn public_func() {
            println!("Called public of custom_mod::nested::public_func");
        }

        #[allow(dead_code)]
        fn private_func() {
            println!("Called custom_mod::nested::private_func");
        }

        pub(in crate::custom_mod) fn public_in_custom_mod() {
            println!("Called custom_mod::nested::public_in_custom_mod");
        }

        // Functions declared using super are only visible within the parent module
        pub(super) fn public_in_super() {
            println!("Called custom_mod::nested::public_in_super");
        }
    }

    pub fn call_public_function_custom_mod() {
        println!("Called custom_mod::call_public_function_custom_mod");
        nested::public_in_custom_mod();
        nested::public_in_super();
    }

    pub(crate) fn public_function_in_crate() {
        println!("Called custom_mod::public_function_in_crate");
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("Called custom_mod::private_nested");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("Called custom_mod::private_nested::restricted_function");
        }
    }
}

fn function() {
    println!("Called function");
}

fn main() {
    function();

    custom_mod::public_func();

    {
        println!("Called custom_mod::public_func named as other_func");
        other_func();
    }

    custom_mod::indirect_func();

    custom_mod::nested::public_func();

    custom_mod::call_public_function_custom_mod();

    custom_mod::public_function_in_crate();

    let open_box: OpenBox = OpenBox {
        content: String::from("Open Box"),
    };

    println!("Contents of open_box: {}", open_box.content);

    #[allow(unused_variables)]
    let close_box = CloseBox::new(String::from("Close Box"));
}
