
pub extern crate r2d2;
pub extern crate beanstalkc;

pub use beanstalkc::BeanstalkcError as Error;
pub use beanstalkc::Beanstalkc as Client;

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
    port: u16,
    watch_list: Vec<String>,
}

impl BeanstalkdConnectionManager {
    /// Creates a new `RedisConnectionManager`.
    ///
    /// See `redis::Client::open` for a description of the parameter
    /// types.
    pub fn new(host: String, port: u16, watch_list: Vec<String>) -> BeanstalkdConnectionManager {
        BeanstalkdConnectionManager { host, port, watch_list }
    }
}

impl r2d2::ManageConnection for BeanstalkdConnectionManager {
    type Connection = Client;
    type Error = Error;

    fn connect(&self) -> Result<Client, Error> {
		let mut conn = Client::new().host(&self.host).port(self.port).connect()?;
        for tube in &self.watch_list {
            conn.watch(&tube)?;
        }
        Ok(conn)
    }

    fn is_valid(&self, conn: &mut Client) -> Result<(), Error> {
        conn.stats()?;
        Ok(())
    }

    fn has_broken(&self, conn: &mut Client) -> bool {
        false
    }
}
