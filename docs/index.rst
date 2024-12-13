Arcaxh Translator Documentation
=============================

.. automodule:: arcaxh_translator
   :members:
   :undoc-members:
   :show-inheritance:

Installation
-----------

To install the Arcaxh translator:

.. code-block:: bash

   pip install arcaxh-translator

Usage
-----

Basic usage example:

.. code-block:: python

   from arcaxh_translator import ArcaxhTranslator
   
   translator = ArcaxhTranslator()
   translation = translator.translate_to_english("arkashir")
   print(translation)
