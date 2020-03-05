from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="rust-test",
    version="1.0",
    rust_extensions=[RustExtension("rust_test.rust_test", binding=Binding.PyO3)],
    packages=["rust_test"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)