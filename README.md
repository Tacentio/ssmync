# ssmync

A command line tool for managing AWS SSM Parameters across multiple regions.

## Prerequisites

Your machine must be authenticated to AWS in some way. Uses the provider chain to resolve credentials.

## Installation

## Usage

```
ssmync 0.1.0
Program to sync SSM Parameter Store entries accross regions. New values for parameters can be
entered once the program is ran to prevent secrets ending up in shell history

USAGE:
    ssmync [OPTIONS] --parameter <PARAMETER> --type-param <TYPE_PARAM>

OPTIONS:
    -d, --dry-run                    Show what changes would be made but don't actually change
                                     anything
    -h, --help                       Print help information
    -p, --parameter <PARAMETER>      Parameter you want to sync
    -r, --regions <REGIONS>          Regions to work on
    -t, --type-param <TYPE_PARAM>    Type of parameter. Used when a new param need to be created
    -v, --value <VALUE>              Value to set parameter to
    -V, --version                    Print version information
```

## Examples

Note: Valid values for `--type` are:

- `SecureString`
- `String`
- `StringList`

The type will be ignored if the parameter is being updated. This probably needs to be fixed.

### Setting a non-sensitive value

```
ssmync --parameter /test --value "non-sensitive-value" -t String
```

### Setting a sensitive value

Sensitive value should not be entered on the command line with `-v` or `--value` as the sensitive value will end up in your shell history. Instead run something like:

```
ssmync --parameter /test -t SecureString
```

You will be prompted to enter the value.

### Limiting region

By default `ssmync` will operate on all regions. To limit regions, specify each one with the `-r` switch.

```
ssmync --parameter /test -t SecureString -r ap-southeast-2 -r us-west-2
```

## Contributing

Any Pull Requests are welcome.

## TODO

- Decide what to do about `--type`, it probably should update exising parameters with the new type if that can be done.
- Add support for custom KMS keys. Currently only uses the default key when specifying `SecureString`
