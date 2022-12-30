from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='test_library',
        version='0.1',
        rust_extensions=[
            RustExtension('test_library', 'Cargo.toml',
                binding=Binding.PyO3)],
            zip_safe=False)