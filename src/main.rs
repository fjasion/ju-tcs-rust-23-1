use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
enum Command {
    Head{n:usize,path:PathBuf},
    Tail{n:usize,path:PathBuf},
}

fn head(path: &Path, n: usize) -> Vec<String>{todo!()}
fn tail(path: &Path, n: usize) -> Vec<String>{todo!()}

fn main() {
    let com = Command::parse();
    let ans = match com {
        Command::Head { n, path } => {head(&path, n)}
        Command::Tail { n , path} => {tail(&path, n)}
    };
    for lin in ans {
        println!("{}",lin)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_math() {
        assert_eq!(1 + 2, 3);
    }
}
