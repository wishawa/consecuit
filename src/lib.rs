mod component;
pub mod components;
mod hook;
pub mod hooks;
mod stores;
mod unmounted_lock;

pub use component::{mount, ComponentBuilder, ComponentValue};
pub use hook::{HookBuilder, HookValue};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
