# Libraries

## Development

Here you can find libraries in any programming language, each one inside its own directory.
All these libraries provides the same Bazel targets:
 * `//libraries/<name>`: the main library target, to be used by consumers
 * `//libraries/<name>:doc`: documentation for the library
 * `//libraries/<name>:cli`: if available, a command line interface for the library functionality
 * `//libraries/<name>:tests`: unittests for the library

Some libraries might provide additional targets, but those are specific to their module.
