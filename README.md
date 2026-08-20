# Zed rst

reST (reStructuredText) syntax highlighting for [Zed](https://github.com/zed-industries/zed).

## Installation

To install the Zed plugin, open the Zed command palette (Cmd-shift-p) and select
`zed: extensions`. This plugin is named `ReST` there, though it may not be at
the top of the list when you search. Find it and install it.

This plugin has an external dependency on the [Esbonio](https://docs.esbon.io/)
language server, which you can get with `pipx install esbonio`.

To have the extension automatically install and update esbonio via `pipx` or
`uv`, enable the `install` setting in your Zed settings:

```json
{
  "lsp": {
    "esbonio": {
      "settings": {
        "install": true
      }
    }
  }
}
```

Esbonio requires Sphinx (and any theme/extensions used by your project) to be
available in its Python environment. If installed via pipx, inject them:

```sh
pipx inject esbonio sphinx sphinx-rtd-theme  # or whichever theme your project uses
```

Alternatively, configure Esbonio to use your project's virtualenv where Sphinx
is already installed (see Esbonio's `sphinx.pythonPath` setting).

## Tree-Sitter

- https://github.com/stsewd/tree-sitter-rst
