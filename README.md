# Backup Rotation

**Rotate backup files based on file name pattern or last modified date**

## Usage

backup-rotation [OPTIONS] <FILES>...

## Arguments

- <FILES>... : The list of files to rotate.

## Options

- --format <FORMAT> : Specify the format of the backup file name. If not provided, the last modified time of the file will be used.
- -h, --help : Print help information.
- -V, --version : Print version information.

## Format

- The --format option allows you to define a custom format for the backup file names. The format string can include the following placeholders:

### Required

%Y : Year
%m : Month
%d : Day
%H : Hour
%M : Minute
%S : Second

### Optional

- %z : Timezone (default is UTC)

If no format is specified, the last modified date and time of the file will be used.

## Examples

```bash
backup-rotation --format "%Y-%m-%d_%H-%M-%S" /path/to/backup/files/*
```

This command will rename and rotate the backup files based on the specified format.

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contact

For any questions or feedback, please open an issue.
