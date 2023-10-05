use log::{debug, error, info, log_enabled, trace, warn, Level};

pub fn shave_the_yak(yak: &mut Yak) {
    trace!("Commencing yak shaving");

    loop {
        match find_a_razor() {
            Ok(razor) => {
                info!("Razor located: {:?}", razor);
                yak.shave(razor);
                break;
            }
            Err(err) => {
                warn!("Unable to locate a razor: {}, retrying", err);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

pub struct Yak {
    name: String,
}

impl Yak {
    pub fn shave(&mut self, razor: Razor) {
        println!("Shaving yak {:?} with {:?}", self.name, razor);
    }
}
#[derive(Debug)]
pub struct Razor;

fn find_a_razor() -> Result<Razor, String> {
    Err("Could not find a razor".to_string())
}

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Trace)
        .init();
    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
    let mut yak = Yak {
        name: "Fred".to_string(),
    };
    shave_the_yak(&mut yak);
}
