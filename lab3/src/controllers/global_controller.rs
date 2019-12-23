use crate::{
    views,
    cli,
    models::services::GlobalDbService
};

pub struct GlobalController {
    service: GlobalDbService
}

impl GlobalController {
    pub fn new(service: GlobalDbService) -> Self {
        Self { service }
    }

    pub fn push_random_entities(&self, opts: cli::PushRandomEntities) {
        let total = self.service.push_random_entities(opts);

        match total {
            Ok(total) => views::display_succesfully_pushed_random_entites(total),
            Err(err) => views::display_err(&err)
        };
    }
}
