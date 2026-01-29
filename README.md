# Arbitrary-Distribution Enumerative Sphere Shaping (AD-ESS)


This repository contains the arbitrary-distribution enumerative sphere shaping (AD-ESS) algorithm published in[^1].
The algorithm is implemented in Rust, but this repository also contains Python bindings.

A second algorithm named reverse trellis shaping (RTS) is also located in this repository.
Unlike AD-ESS it uses energy based ordering of the sequences and thus always has minimal rate loss.
Its complexity is the same as Laroias 1st algorithm[^2].

- Rust Documentation [TODO: link]
- Python Documentation [TODO: link]

[^1]: https://arxiv.org/pdf/2512.16808.

[^2]: R. Laroia, N. Farvardin and S. A. Tretter, "On optimal shaping of multidimensional constellations," in IEEE Transactions on Information Theory, vol. 40, no. 4, pp. 1044-1056, July 1994, doi: 10.1109/18.335969.

## Installation / Building

Install the Rust or Python library from crates.io or pypi.org, respectively.

Please refer to the [Rust](./rust/README.md) or [Python](./python/README.md) README for details.

## Citing this work

If you use this library in your research you can reference the following publication.

[TODO: paper reference]

```tex
@inproceedings{extension_rrl2026,
  title={An Extension of Enumerative Sphere Shaping for Arbitrary Channel Input Distributions},
  author={Ritter, Frederik and Rode, Andrej and Schmalen, Laurent},
  booktitle={Proceedings of the International Zurich Seminar on Information and Communication (IZS 2026)},
  volume={TODO},
  number={TODO},
  year={2024}
}
```

## Acknowledgments

This work has received funding from the European Research Council (ERC) under the European Union’s Horizon 2020 research and innovation programme (grant agreement No. 101001899) and the Deutsche Forschungsgemeinschaft (DFG, German Research Foundation) – Grant 555885380.
