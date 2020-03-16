![Travis (.org)](https://img.shields.io/travis/urschrei/minimal_cross_manylinux)

# Building `manylinux`-compatible Rust Binaries using `cross`

This is a sample repo demonstrating the building of `manylinux2010`-compatible Rust binaries using `cross`. It accompanies [this Medium article](https://medium.com/@urschrei/building-manylinux-compatible-rust-binaries-for-use-in-python-wheels-d5d943619af2?sk=22b0891d8f7403279561de68f3bcbd98), which gives more detail.

## What This Repo Does
- Provides an example C-compatible external function
- Provides an example [Cargo.toml](Cargo.toml) which:
    - builds a [`cdylib`](https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/cdylib-crates-for-c-interoperability.html)
    - uses _fat_ link-time-optimization
    - uses a single codegen unit
    - uses [`cbindgen`](https://github.com/eqrion/cbindgen) to generate a C-compatible header
- Uses [`cross`](https://github.com/rust-embedded/cross) and its [metadata file](Cross.toml) to build a `manylinux2010 x86_64` compatible dynamic library
- embeds the [correct `rpath` data](https://stackoverflow.com/a/19147134/416626) for use when building Python extensions
- strips binaries

## A Note on Building Python Extensions
This is a large and complex subject, but three things must be done:

1. Your Rust library's `rpath` info must be set correctly (see above)
2. `setup.py` must have `extra_link_args` set correctly
3. Your Rust library (`rust_dylib`) must _additionally_ be available in a directory that is part of `LD_LIBRARY_PATH`

Assuming a directory structure for a Python library named `my_python_library` that looks like:

- `setup.py`
- `setup.cfg`
- `manifest.in`
- my_python_library
    - `my_python_library.cutil`
    - `rust_dylib`
    - `header.h`

where `my_python_library.cutil` is a `pyx` file which provides a thin wrapper around the exported Rust function(s) in `rust_dylib`,
The following `setup.py` configuration should allow you to build a wheel:

```python
from setuptools import setup, find_packages, Distribution, Extension
from Cython.build import cythonize


class BinaryDistribution(Distribution):
    def is_pure(self):
        return False


if "linux" in sys.platform:
    # from http://stackoverflow.com/a/10252190/416626
    # the $ORIGIN trick is not perfect, though
    ldirs = ["-Wl,-rpath", "-Wl,$ORIGIN"]
if sys.platform == "darwin":
    ldirs = ["-Wl,-rpath", "-Wl,@loader_path/"]

extensions = Extension(
    # pyx file which wraps your rust binary
    # users import it:
    # from python_library.cutil import double
    "my_python_library.cutil",
    sources=["my_python_library/cutil.pyx"],
    libraries=["rust_dylib"],
    include_dirs=["my_python_library"],
    library_dirs=["my_python_library"],
    extra_link_args=ldirs,
)
extensions = cythonize([extensions,])

setup(
    # other keywords are mandatory but not included here
    name="my_python_library",
    include_package_data=True,
    distclass=BinaryDistribution,
    packages=find_packages(),
    ext_modules=extensions,
)
```

Once this is done, you can build your wheel. You are strongly advised to use 
[Multibuild](https://github.com/matthew-brett/multibuild) in order to do this, as it's the most thorough and well-tested system for generating binary wheels. In order for multibuild's final step – repairing and correctly tagging your wheel – to work, you are _strongly_ advised to do the following:

- Create a directory such as `/usr/local/lib` and **add** it to `LD_LIBRARY_PATH`:
    - `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib`
- **copy** `rust_dylib` into the directory you just created, as this is where `auditwheel` searches for libraries to bundle.


# License

[The Blue Oak Model License 1.0.0](LICENSE.md)
