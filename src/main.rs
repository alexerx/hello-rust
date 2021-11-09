// mod try_mod; // 加载foo.rs文件中的内容
// mod try_vec;
// mod try_enum;
// mod try_trait;
mod try_lifetime;


fn main() {
    // try_mod::foo_fn();
    // try_vec::try_enum_in_vec();
    // try_enum::try_enum_destruction();
    // try_trait::try_trait_fn();
    try_lifetime::try_lifetime_fn();
}
