reStructuredText
----------------

|rst| *supports* inline markup like:

- **strong**
- *emphasis*
- ``literal text``

It also supports extending the markup using directives.

.. note::

   I'm a directive! There are a couple of built-in directives:

   - image
   - warning
   - etc

   Find all of them in the documentation_

   .. _documentation: https://docutils.sourceforge.io/docs/ref/rst/directives.html


This is a |rst| grammar for tree-sitter_, for contributing or report any bugs
check https://github.com/stsewd/tree-sitter-rst/.

.. _tree-sitter: https://tree-sitter.github.io/tree-sitter/

.. |rst| replace:: reStructuredText
