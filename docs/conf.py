import os
import sys
sys.path.insert(0, os.path.abspath('..'))

project = 'Arcaxh Translator'
copyright = '2024'
author = 'Your Name'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.napoleon'
]

html_theme = 'sphinx_rtd_theme'