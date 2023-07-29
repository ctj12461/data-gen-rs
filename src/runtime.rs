use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

use rayon::ThreadPoolBuilder;

use crate::config::Config;
use crate::task::{Generator, Solver};

pub struct Executor {
    name: String,
    config: Vec<Config>,
}

pub struct GeneratorEnvironment {
    pub input: BufWriter<File>,
}

pub struct SolverEnvironment {
    pub input: BufReader<File>,
    pub output: BufWriter<File>,
}

impl Executor {
    pub fn new<S: ToString>(name: S) -> Self {
        Self {
            name: name.to_string(),
            config: Vec::new(),
        }
    }

    pub fn testcase(&mut self, config: Config) {
        self.config.push(config)
    }

    pub fn run<G, S>(self, generator: G, solver: S, thread_num: usize)
    where
        G: Generator,
        S: Solver,
    {
        let generator = &generator;
        let solver = &solver;
        let name = &self.name;
        let pool = ThreadPoolBuilder::new()
            .num_threads(thread_num.max(1))
            .build()
            .unwrap();

        pool.scope(|s| {
            for config in self.config {
                s.spawn(move |_| {
                    let input_path = config.generate_name(name, "in");
                    let output_path = config.generate_name(name, "out");

                    let env = GeneratorEnvironment::new(&input_path);
                    generator.run(env, &config);

                    let env = SolverEnvironment::new(input_path, output_path);
                    solver.run(env, &config);

                    match config.subtask_id() {
                        Some(subtask) => eprintln!("Testcase {subtask}-{} Generated", config.id),
                        None => eprintln!("Testcase {} Generated", config.id),
                    }
                });
            }
        });
    }
}

impl GeneratorEnvironment {
    pub fn new<P: AsRef<Path>>(input_path: P) -> Self {
        let mut open_options = OpenOptions::new();
        open_options.write(true).create(true).truncate(true);
        let input = open_options.open(input_path.as_ref()).unwrap();

        Self {
            input: BufWriter::new(input),
        }
    }
}

impl SolverEnvironment {
    pub fn new<P: AsRef<Path>>(input_path: P, output_path: P) -> Self {
        let mut open_options = OpenOptions::new();
        open_options.read(true).write(true).create(true);
        let input = open_options.open(input_path.as_ref()).unwrap();
        open_options.truncate(true);
        let output = open_options.open(output_path.as_ref()).unwrap();

        Self {
            input: BufReader::new(input),
            output: BufWriter::new(output),
        }
    }
}
