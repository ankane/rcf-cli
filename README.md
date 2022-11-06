# Random Cut Forest CLI

Command-line interface for [Random Cut Forest](https://github.com/aws/random-cut-forest-by-aws) anomaly detection

[![Build Status](https://github.com/ankane/rcf-cli/workflows/build/badge.svg?branch=master)](https://github.com/ankane/rcf-cli/actions)

## Installation

Download the latest version:

- Linux - [x86_64]() or [arm64]()
- Mac - [x86_64]() or [arm64]()
- Windows - [x86_64]()

You can also install it with Homebrew:

```sh
brew install ankane/brew/rcf
```

## Getting Started

Prepare a CSV. Each row should be a data point. Here’s [an example](https://raw.githubusercontent.com/ankane/rcf-cli/master/example-data/input.csv) with three dimensions:

```text
$ tail input.csv
5.0001,0.0006,-0.0084
-4.9998,0.0041,0.0025
-5.0096,-0.0085,-0.0007
4.9980,-0.0049,-0.0029
5.0083,0.0078,-0.0029
0.0001,-0.0052,0.0003
5.0051,0.0081,-0.0068
5.0044,-0.0030,-0.0016
-5.0028,-0.0067,-0.0056
4.9921,0.0092,0.0049
```

And run:

```sh
rcf < input.csv > output.csv
```

The program outputs the anomaly score as a new column. Anomalous data points are given a larger score.

```text
$ tail output.csv
5.0001,0.0006,-0.0084,0.7980522827802555
-4.9998,0.0041,0.0025,0.7071138288972222
-5.0096,-0.0085,-0.0007,1.018823150444355
4.9980,-0.0049,-0.0029,0.7408316990007481
5.0083,0.0078,-0.0029,0.8277566916765301
0.0001,-0.0052,0.0003,2.8337001378872664
5.0051,0.0081,-0.0068,0.7709170753009097
5.0044,-0.0030,-0.0016,0.7204096423354174
-5.0028,-0.0067,-0.0056,0.8198039786393572
4.9921,0.0092,0.0049,0.9185893539878859
```

## Options

```text
  -d, --delimiter <DELIMITER>        The character used as a field delimiter [default: ,]
      --header-row                   Pass if the data contains a header row
  -n, --number-of-trees <TREES>      Number of trees to use in the forest [default: 100]
  -r, --random-seed <RANDOM_SEED>    Random seed to use [default: 42]
  -s, --sample-size <SAMPLE_SIZE>    Points to keep in sample for each tree [default: 256]
  -c, --shingle-cyclic               Use cyclic shingles instead of linear shingles
  -g, --shingle-size <SHINGLE_SIZE>  Shingle size to use [default: 1]
  -w, --window-size <WINDOW_SIZE>    Window size of the sample or 0 for no window [default: 0]
  -h, --help                         Print help information
  -V, --version                      Print version information
```

## References

- [Robust Random Cut Forest Based Anomaly Detection On Streams](https://proceedings.mlr.press/v48/guha16.pdf)

## Credits

This project is modeled after the [Java CLI](https://github.com/aws/random-cut-forest-by-aws/tree/main/Java#build-command-line-cli-usage).

## History

View the [changelog](CHANGELOG.md)

## Contributing

Everyone is encouraged to help improve this project. Here are a few ways you can help:

- [Report bugs](https://github.com/ankane/rcf-cli/issues)
- Fix bugs and [submit pull requests](https://github.com/ankane/rcf-cli/pulls)
- Write, clarify, or fix documentation
- Suggest or add new features

To get started with development:

```sh
git clone https://github.com/ankane/rcf-cli.git
cd rcf-cli
cargo run --release < example-data/input.csv
```
