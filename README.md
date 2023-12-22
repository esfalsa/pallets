# pallets
> Command-line download manager for NationStates daily data dumps

## Install

```sh
cargo install --git https://github.com/esfalsa/pallets
```

## Usage

Some of the more common use cases are shown below. See `pallets --help` for more detailed information.

### Download a daily data dump

```sh
pallets download --user Esfalsa --type regions --date 2022-04-01
```

### Delete a daily data dump

```sh
pallets delete --type regions --date 2022-04-01
```

### List downloaded daily data dumps

```sh
pallets list
```

## Contributing

Contributions, bug reports, and feature requests are welcome! Feel free to create an [issue](https://github.com/esfalsa/pallets/issues) or [pull request](https://github.com/esfalsa/pallets/pulls).

## License

[AGPLv3](./LICENSE)
