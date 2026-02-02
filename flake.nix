{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nix-community/naersk/master";
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      ...
    }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;

      pypkgs = pkgs.python311Packages;
      python = pkgs.python311;

      naersk-lib = pkgs.callPackage naersk { };

      # PyADTS
      # inspired by https://github.com/litchipi/nix-build-templates/blob/6e4961dc56a9bbfa3acf316d81861f5bd1ea37ca/rust/maturin.nix
      wheel_tail = "cp311-cp311-linux_x86_64";
      pyadess_cfg = rec {
        pname = "pyadess";
        version = "0.1.0";
        wheel_file = "${pname}-${version}-${wheel_tail}.whl";
      };

      pyadess_wheel =
        (naersk-lib.buildPackage {
          src = ./python;
          nativeBuildInputs = [
            pkgs.m4
            python
            pkgs.tree
          ];
          preBuild = ''
            mkdir ../rust
            cp -r ${self}/rust/* ../rust
          '';
        }).overrideAttrs
          (old: {
            nativeBuildInputs = old.nativeBuildInputs ++ [ pkgs.maturin ];
            buildPhase = old.buildPhase + ''
              maturin build --release --offline --target-dir ./target
            '';
            installPhase = old.installPhase + ''
              cp target/wheels/${pyadess_cfg.wheel_file} $out/
            '';
          });

      pyadess = pypkgs.buildPythonPackage {
        pname = pyadess_cfg.pname;
        version = pyadess_cfg.version;
        format = "wheel";
        src = "${pyadess_wheel}/${pyadess_cfg.wheel_file}";
      };
    in
    {
      packages.x86_64-linux = {
        default = pyadess;
        pyadess-doc = pkgs.stdenv.mkDerivation {
          name = "pyadess-documentation";
          buildInputs = [
            python
            pypkgs.numpy
            pypkgs.pdoc
            pyadess
          ];
          src = ./.;
          buildPhase = ''
            mkdir -p $out
            pdoc pyadess --math -o $out

            sed -i -e 's~<span class="n">Union<\/span><span class="p">\[<\/span><span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">_typing<\/span><span class="o">\.<\/span><span class="n">_array_like<\/span><span class="o">\.<\/span><span class="n">_Buffer<\/span><span class="p">,<\/span> <span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">_typing<\/span><span class="o">\.<\/span><span class="n">_array_like<\/span><span class="o">\.<\/span><span class="n">_SupportsArray<\/span><span class="p">\[<\/span><span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">dtype<\/span><span class="p">\[<\/span><span class="n">Any<\/span><span class="p">\]\],<\/span> <span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">_typing<\/span><span class="o">\.<\/span><span class="n">_nested_sequence<\/span><span class="o">\.<\/span><span class="n">_NestedSequence<\/span><span class="p">\[<\/span><span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">_typing<\/span><span class="o">\.<\/span><span class="n">_array_like<\/span><span class="o">\.<\/span><span class="n">_SupportsArray<\/span><span class="p">\[<\/span><span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">dtype<\/span><span class="p">\[<\/span><span class="n">Any<\/span><span class="p">\]\]\],<\/span> <span class="nb">complex<\/span><span class="p">,<\/span> <span class="nb">bytes<\/span><span class="p">,<\/span> <span class="nb">str<\/span><span class="p">,<\/span> <span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">_typing<\/span><span class="o">\.<\/span><span class="n">_nested_sequence<\/span><span class="o">\.<\/span><span class="n">_NestedSequence<\/span><span class="p">\[<\/span><span class="nb">complex<\/span> <span class="o">|<\/span> <span class="nb">bytes<\/span> <span class="o">|<\/span> <span class="nb">str<\/span><span class="p">\]\]<\/span>~<span class="n">numpy.ArrayLike<\/span>~g' "$out/pyadess.html"

            sed -i -e 's~<span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">ndarray<\/span><span class="p">\[<\/span><span class="nb">tuple<\/span><span class="p">\[<\/span><span class="n">typing<\/span><span class="o">\.<\/span><span class="n">Any<\/span><span class="p">,<\/span> <span class="o">\.\.\.<\/span><span class="p">\],<\/span> <span class="n">numpy<\/span><span class="o">\.<\/span><span class="n">dtype<\/span>\(.*\)\]<\/span>~numpy.ndarray\1~g' "$out/pyadess.html"
          '';
        };
      };
      devShells.x86_64-linux = {
        default =
          with pkgs;
          mkShell {
            venvDir = "./.venv";
            buildInputs = [
              # General
              pre-commit

              # Rust
              cargo
              rustc
              rustfmt
              rustPackages.clippy
              m4

              # Maturin
              gcc
              maturin

              # Python
              pypkgs.python
              pypkgs.venvShellHook
              pypkgs.numpy
              pypkgs.pdoc # for documentation
            ];
            postVenvCreation = ''
              unset SOURCE_DATE_EPOCH
            '';
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
            PYO3_PYTHON = "python";
          };
        test_build =
          with pkgs;
          mkShell {
            buildInputs = [
              python
              pypkgs.numpy
              pyadess
            ];
          };
      };
    };
}
