# converter
A simple unit converter. Parses an input expression (with units), and prints the computed result. Available operators are `+-*/`, with familiar precedence and association.

Units (optionally followed by powers) are acceptable primary values, and are grouped such that `1 kg m/s2` evaluates to the same result as `1N`. Most standard SI units are implemented, along with some imperial and other common units.
---
##Usage
`cargo run -- [-i | "expression"]`

A provided string will be parsed and evaluated, while `-i` evaluates expressions on loop.
