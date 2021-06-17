mod component;
pub mod components;
mod executor;
mod hook;
pub mod hooks;
mod stores;
mod unmounted_lock;
mod props;

pub use component::{mount, ComponentBuilder, ComponentReturn, ContainerReturn};
pub use hook::{HookBuilder, HookReturn};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
