#[cfg(test)]
mod tests {}
use clap::ValueEnum;

#[derive(Debug, Clone, ValueEnum)]
pub enum Part {
    Part1,
    Part2,
}
