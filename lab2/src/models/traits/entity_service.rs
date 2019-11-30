pub trait EntityService {
    type Entity: From<pg::row::Row>;
}

pub trait UpdatableEntityService {
    type EntityUpd;
}

pub trait CreatableEntityService {
    type EntityNew;
}
