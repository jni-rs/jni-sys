#![allow(clippy::all)]
#![allow(unknown_lints)]
#![allow(function_casts_as_integer)]

macro_rules! tests {
    ($name:ident for $($lib:ident)and*) => {
        mod $name {
            $(
            use $lib::*;
            )*
            include!(concat!(env!("OUT_DIR"), "/", stringify!($name), ".rs"));
            pub fn exec() {
                eprintln!(concat!("TESTS FOR '", stringify!($name), "'"));
                main();
            }
        }

        #[cfg(test)]
        #[test]
        fn $name() {
            $name::exec();
        }
    };
}

tests!(jni for jni_sys);

fn main() {
    jni::exec();
}
