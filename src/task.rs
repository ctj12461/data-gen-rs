use std::path::PathBuf;
use std::process::Command;

use crate::config::Config;
use crate::runtime::{GeneratorEnvironment, SolverEnvironment};

pub trait Generator: Send + Sync {
    fn run(&self, env: GeneratorEnvironment, config: &Config);
}

pub trait Solver: Send + Sync {
    fn run(&self, env: SolverEnvironment, config: &Config);
}

impl<F> Generator for F
where
    F: Fn(GeneratorEnvironment, &Config) + Send + Sync,
{
    fn run(&self, env: GeneratorEnvironment, config: &Config) {
        (*self)(env, config);
    }
}

impl<F> Solver for F
where
    F: Fn(SolverEnvironment, &Config) + Send + Sync,
{
    fn run(&self, env: SolverEnvironment, config: &Config) {
        (*self)(env, config);
    }
}

pub struct ExtrernalSolver {
    executable: PathBuf,
}

impl ExtrernalSolver {
    pub fn new<P: Into<PathBuf>>(executable: P) -> Self {
        Self {
            executable: executable.into(),
        }
    }
}

impl Solver for ExtrernalSolver {
    fn run(&self, env: SolverEnvironment, _config: &Config) {
        Command::new(&self.executable)
            .stdin(env.input.into_inner())
            .stdout(env.output.into_inner().unwrap())
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
    }
}
