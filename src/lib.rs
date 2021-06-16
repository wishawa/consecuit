mod component;
pub mod components;
mod hook;
pub mod hooks;
mod stores;
mod holes;
mod unmounted_lock;

pub use component::{mount, ComponentBuilder, ComponentReturn};
pub use hook::{HookBuilder, HookReturn};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
