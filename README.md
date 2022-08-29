## Sucking YQTB

English | [中文](./docs/README_zh.md)

A simple program for the fucking silly epidemic reporting system.

### Usage

Rename `at_school.example.toml`(if you have returned to the school) or `other.example.toml`(if not) to `config.toml` and modify it with your information.

If you don't want to write your username and password in the config file, you can delete `[basic]` and the two lines below, or just delete `username` or `password`, or both, whatever you want.

Then open a terminal and run:

```bash
./syq
```

If you want to keep reporting until it succeeds, run:

```bash
./syq -r
```

If you don't write your username and password in the config file, you must add them when you use the program:

```bash
./syq -u <username> -p <password>
```

More information can be run:

```bash
./syq -h
```
