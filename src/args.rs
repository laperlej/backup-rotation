use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(required=true)]
    pub files: Vec<String>,

    #[arg(long)]
    pub format: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_args() {
        let args = Args::parse_from(["test", "file1", "file2"]);
        assert_eq!(args.files, vec!["file1".to_string(), "file2".to_string()]);
        assert_eq!(args.format, None);
    }

    #[test]
    fn test_args_with_format() {
        let args = Args::parse_from(["test", "file1", "file2", "--format", "pg_%Y-%m-%d_%H-%M-%S.tar"]);
        assert_eq!(args.files, vec!["file1".to_string(), "file2".to_string()]);
        assert_eq!(args.format, Some("pg_%Y-%m-%d_%H-%M-%S.tar".to_string()));
    }
}
