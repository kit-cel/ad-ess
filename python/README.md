# Python bindings for AD-ESS

Python bindings for arbitrary-distribution enumerative sphere shaping (AD-ESS).

For examples see [the example script](./example.py). For documentation see [TODO](TODO).

## Installation

There may currently be some issues installing PyAD-ESS on Windows as AD-ESS uses GMP which can not easily be built on Windows.

### Using PIP

Type `pip install pyadess` into your favourite command line.

### From Source

1. Make sure that Rust and its package manager `cargo` are installed
2. Clone this repository
3. Create a virtual python environment in a folder of your choice (e.g. `python -m venv $VENV_NAME`)
4. Activate the virtual environment (e.g. `cd $VENV_NAME; source bin/activate` if you are using Bash)
5. Install the `pyadess` package with `pip`: `pip install $YOUR_PATH_TO/adess/python`
	- If this fails, your `pip` may be to old. Try `pip install --upgrade pip`

## Development

Building can be done according to: https://pyo3.rs/v0.17.3/getting_started.html

TLDR: `pip install maturin; maturin develop`

### Documentation

Documentation can be built using (a sufficiently recent version of) `pdoc`.
First run `maturin develop` or `pip install .` next build the docs with `pdoc --math pyadess`.
A scrip automating this process is located in `mk_doc.sh`.
This script additionally replaces some ugly numpy type hints with nice ones.

## Project content

- `src/lib.rs`: PyO3 Rust to python bindings
- `pyadess.pyi`: Python function type hints
