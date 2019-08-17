
pub extern crate r2d2;
pub extern crate beanstalkd;

pub use beanstalkd::error::BeanstalkdError as Error;
use beanstalkd::Beanstalkd as Client;

/// An `r2d2::ConnectionManager` for `redis::Client`s.
///
/// ## Example
///

/// ```
/// extern crate r2d2_redis;
///
/// use std::ops::DerefMut;
/// use std::thread;
///
/// use r2d2_redis::{r2d2, redis, RedisConnectionManager};
///
/// fn main() {
///     let manager = RedisConnectionManager::new("redis://localhost").unwrap();
///     let pool = r2d2::Pool::builder()
///         .build(manager)
///         .unwrap();
///
///     let mut handles = vec![];
///
///     for _i in 0..10i32 {
///         let pool = pool.clone();
///         handles.push(thread::spawn(move || {
///             let mut conn = pool.get().unwrap();
///             let reply = redis::cmd("PING").query::<String>(conn.deref_mut()).unwrap();
///             // Alternatively, without deref():
///             // let reply = redis::cmd("PING").query::<String>(&mut *conn).unwrap();
///             assert_eq!("PONG", reply);
///         }));
///     }
///
///     for h in handles {
///         h.join().unwrap();
///     }
/// }
/// ```
///
#[derive(Debug)]
pub struct BeanstalkdConnectionManager {
    host: String, 
    port: u16
}

impl BeanstalkdConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See `redis::Client::open` for a description of the parameter
    /// types.
    pub fn new(host: String, port: u16) -> BeanstalkdConnectionManager {
        BeanstalkdConnectionManager { host, port }
    }
}

impl r2d2::ManageConnection for BeanstalkdConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
        Client::connect(&self.host, self.port)
    }

    fn is_valid(&self, conn: &mut Client) -> Result<(), Error> {
        conn.stats()?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Client) -> bool {
        false
    }
}
