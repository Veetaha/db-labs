

/// An `r2d2::ManageConnection` for `postgres::Client`s.
///
/// ## Example
///
/// ```no_run
/// use std::thread;
/// use postgres::{NoTls, Client};
/// use r2d2_postgres::PgConnMgr;
///
/// fn main() {
///     let manager = PgConnMgr::new(
///         "host=localhost user=postgres".parse().unwrap(),
///         NoTls,
///     );
///     let pool = r2d2::Pool::new(manager).unwrap();
///
///     for i in 0..10i32 {
///         let pool = pool.clone();
///         thread::spawn(move || {
///             let mut client = pool.get().unwrap();
///             client.execute("INSERT INTO foo (bar) VALUES ($1)", &[&i]).unwrap();
///         });
///     }
/// }
/// ```
#[derive(Debug)]
pub struct PgConnMgr<T> {
    config: pg::Config,
    tls_connector: T,
}

impl<T> PgConnMgr<T>
where
    T: pg::tls::MakeTlsConnect<pg::Socket> + Clone + 'static + Sync + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as pg::tls::TlsConnect<pg::Socket>>::Future: Send,
{
    /// Creates a new `PgConnMgr`.
    pub fn new(config: pg::Config, tls_connector: T) -> Self {
        Self { config, tls_connector, }
    }
}

impl<T> r2d2::ManageConnection for PgConnMgr<T>
where
    T: pg::tls::MakeTlsConnect<pg::Socket> + Clone + 'static + Sync + Send,
    T::TlsConnect: Send,
    T::Stream: Send,
    <T::TlsConnect as pg::tls::TlsConnect<pg::Socket>>::Future: Send,
{
    type Connection = pg::Client;
    type Error = pg::Error;

    fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.config.connect(self.tls_connector.clone())
    }

    fn is_valid(&self, client: &mut Self::Connection) -> Result<(), Self::Error> {
        client.simple_query("").map(|_| ())
    }

    fn has_broken(&self, client: &mut Self::Connection) -> bool {
        client.is_closed()
    }
}
