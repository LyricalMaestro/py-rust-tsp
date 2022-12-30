from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="travel_salesman_problem",
    version="0.1",
    rust_extensions=[
        RustExtension(
            "travel_salesman_problem", "Cargo.toml", binding=Binding.PyO3
        )
    ],
    zip_safe=False
)