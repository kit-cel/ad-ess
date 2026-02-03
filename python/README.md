# Python bindings for AD-ESS

An implementation of arbitrary-distribution enumerative sphere shaping (AD-ESS)[^1].
AD-ESS is am extension of the enumerative sphere shaping (ESS)[^2] algorithm that maps uniformly distributed bits to amplitudes with a distribution tailored to the additive white Gaussian noise (AWGN) channel.
In AD-ESS the output distribution of the amplitudes is adaptable, making the algorithm suitable for a wide range of channels.
The algorithm is implemented in Rust, but this package contains Python bindings for the Rust library.

A second algorithm named reverse trellis shaping (RTS) is also implemented.
Unlike AD-ESS it uses energy based ordering of the sequences and thus always has minimal rate loss.
Its complexity is the same as Laroias 1st algorithm[^3].

[^1]: https://arxiv.org/pdf/2512.16808.

[^2]: F. M. J. Willems and J. J. Wuijts, "A pragmatic approach to shaped coded modulation," in Proc. IEEE Symp. on Commun. and Veh. Technol. in the Benelux, 1993.

[^3]: R. Laroia, N. Farvardin and S. A. Tretter, "On optimal shaping of multidimensional constellations," in IEEE Trans. Inf. Theory, vol. 40, no. 4, pp. 1044-1056, July 1994, doi: 10.1109/18.335969.

For examples see [the example script](./example.py). For documentation see [here](https://kit-cel.github.io/ad-ess/pyadess.html).

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
