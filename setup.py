from setuptools import setup, find_packages

setup(
    name="arcaxh_translator",
    version="0.1.0",
    packages=find_packages(),
    install_requires=[
        "maturin>=1.0,<2.0",
    ],
)
