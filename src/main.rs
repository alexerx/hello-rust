// mod try_mod; // 加载foo.rs文件中的内容
// mod try_vec;
// mod try_enum;
// mod try_trait;
// mod try_lifetime;
// mod try_closure;
// mod try_iter;
// mod try_smart_pointer;
mod try_concurrent;


fn main() {
    // try_mod::foo_fn();
    // try_vec::try_enum_in_vec();
    // try_enum::try_enum_destruction();
    // try_trait::try_trait_fn();
    // try_lifetime::try_lifetime_fn();
    // try_closure::try_closure_fn();
    // try_iter::try_iter_fn();

    // try_smart_pointer::run()

    try_concurrent::run();
}
