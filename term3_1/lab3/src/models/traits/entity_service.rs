use diesel::{Insertable, Queryable, Identifiable};

pub trait QueryableEntityService {
    type Entity: Queryable;
}

pub trait UpdatableEntityService {
    type EntityUpd: Identifiable;
}

pub trait CreatableEntityService {
    type EntityNew: Insertable;
}
