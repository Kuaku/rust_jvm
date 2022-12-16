mod util;
mod class_file;
mod jvm;
use std::error::Error;
use std::fs;
use util::file;

pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let class_binary = fs::read(config.filename)?;
    let class_file = class_file::parse_file(&mut file::File::new(class_binary));
    let main_method = jvm::find_methods_by_name(&class_file, Box::new(&class_file), "main".as_bytes().to_vec())[0];
    let mut jvm = jvm::JVM::new()
    .register(Box::new(jvm::static_module::StaticModule{}))
    .register(Box::new(jvm::integer_module::IntegerModule {}));
    jvm.execute_main_method(&class_file, main_method);
    Ok(())
}
pub struct Config {
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let filename = args[1].clone();

        return Ok(Config {
            filename,
        });
    }
}