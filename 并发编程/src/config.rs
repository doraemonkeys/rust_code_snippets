use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

#[derive(Clone, Debug)]
pub struct Config {
    pub ip: String,
    pub port: u16,
}

impl Config {
    pub fn new(ip: String, port: u16) -> Self {
        Self { ip, port }
    }

    pub fn access<M: AccessMode>(self) -> RWAccess<Config, M> {
        RWAccess {
            config: Arc::new(RwLock::new(self)),
            _phantom: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    pub fn read_only(self) -> RWAccess<Config, ReadAccess> {
        self.access()
    }

    pub fn read_write(self) -> RWAccess<Config, WriteAccess> {
        self.access()
    }
}

#[derive(Clone, Debug)]
pub struct ReadAccess;
#[derive(Clone, Debug)]
pub struct WriteAccess;

// 特性定义，用于抽象读写操作
pub(crate) trait AccessMode {}

// pub(crate)是一个可见性修饰符，用来指示一个项（如函数、结构体、枚举、模块等）是公共的，
// 但只在当前crate（包）内。Rust中的crate是一个编译单元，可以是一个库或一个可执行文件。
// 当你使用pub(crate)修饰一个项时，这意味着这个项对整个crate内是可见的，
// 但对于这个crate之外的代码则是私有的。这在你想要隐藏实现细节，
// 但在crate内部的多个模块间共享代码时非常有用。

impl AccessMode for ReadAccess {}
impl AccessMode for WriteAccess {}

#[derive(Clone, Debug)]
pub struct RWAccess<T: ?Sized, M: AccessMode> {
    config: Arc<RwLock<T>>,
    _phantom: std::marker::PhantomData<M>,
}

// 读写的公共方法
impl<T, M: AccessMode> RWAccess<T, M> {
    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        self.config.read().unwrap()
    }

    pub fn clone_reader(&self) -> RWAccess<T, ReadAccess> {
        RWAccess {
            config: self.config.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

// 写的私有方法
impl<T> RWAccess<T, WriteAccess> {
    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.config.write().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new("192.168.1.111".to_string(), 8080);
        let cnf_writer = config.read_write();
        assert_eq!(cnf_writer.read().ip, "192.168.1.111");

        let config_reader = cnf_writer.clone_reader();

        cnf_writer.write().ip = "hhhhh".to_string();
        assert_eq!(config_reader.read().ip, "hhhhh");

        let cnf_writer2 = cnf_writer.clone();
        let config_reader2 = config_reader.clone();
        assert_eq!(cnf_writer2.read().ip, config_reader2.read().ip, "hhhhh");
    }
}
