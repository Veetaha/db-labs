use diesel::{pg::Pg, deserialize::QueryableByName};

pub trait EntityService {
    type Entity: QueryableByName<Pg>;
}
