# Arbitrary-Distribution Enumerative Sphere Shaping (AD-ESS)

An implementation of arbitrary-distribution enumerative sphere shaping (AD-ESS)[^1].
AD-ESS is am extension of the enumerative sphere shaping (ESS)[^2] algorithm that maps uniformly distributed bits to amplitudes with a distribution tailored to the additive white Gaussian noise (AWGN) channel.
In AD-ESS the output distribution of the amplitudes is adaptable, making the algorithm suitable for a wide range of channels.
The algorithm is implemented in Rust, but this repository also contains Python bindings.

A second algorithm named reverse trellis shaping (RTS) is also implemented.
Unlike AD-ESS it uses energy based ordering of the sequences and thus always has minimal rate loss.
Its complexity is the same as Laroias 1st algorithm[^3].

- Rust Documentation [TODO: link]
- [Python Documentation](https://kit-cel.github.io/ad-ess/pyadess.html)

[^1]: https://arxiv.org/pdf/2512.16808.

[^2] F. M. J. Willems and J. J. Wuijts, "A pragmatic approach to shaped coded modulation," in Proc. IEEE Symp. on Commun. and Veh. Technol. in the Benelux, 1993.

[^3]: R. Laroia, N. Farvardin and S. A. Tretter, "On optimal shaping of multidimensional constellations," in IEEE Trans. Inf. Theory, vol. 40, no. 4, pp. 1044-1056, July 1994, doi: 10.1109/18.335969.

## Installation / Building

Install the Rust or Python library from crates.io or pypi.org, respectively.

Please refer to the [Rust](./rust/README.md) or [Python](./python/README.md) README for details.

## Citing this work

If you use this library in your research you can reference the following publication.

```tex
@inproceedings{extension_rrl2026,
  title={An Extension of Enumerative Sphere Shaping for Arbitrary Channel Input Distributions},
  author={Ritter, Frederik and Rode, Andrej and Schmalen, Laurent},
  booktitle={Proceedings of the International Zurich Seminar on Information and Communication (IZS 2026)},
  year={2026}
}
```

## Acknowledgments

This work has received funding from the European Research Council (ERC) under the European Union’s Horizon 2020 research and innovation programme (grant agreement No. 101001899) and the Deutsche Forschungsgemeinschaft (DFG, German Research Foundation) – Grant 555885380.

# License

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
