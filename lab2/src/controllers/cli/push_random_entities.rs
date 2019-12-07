use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct PushRandomEntities {
    /// Minimum amount of entities to creater per table
    #[structopt(name = "min", long, default_value = "10")]
    pub min_amount: u32,

    /// Maximum amount of entities to create per table
    #[structopt(name = "max", long, default_value = "20")]
    pub max_amount: u32

}
