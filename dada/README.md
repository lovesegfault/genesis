# dada

This is a simple genetic algorithm that evolves to match some text. It has no
practical usage that I'm aware of, and exists solely as a toy for me to play
with genetic algorithms.

The algorithm implemented is roughly what's described in the paper
["Path Planning for a UAV Performing Reconnaissance of Static Ground Targets in Terrain"][obermeyer-2009].
The core differences are that it allows for repeated elements in the solutions,
and that the mutation probability is different.

## building
There are no external dependencies, so the project can be built and run with
`cargo build`, `cargo run`, respectively.

Additionally, a Nix file is provided, and it can be used with `nix-build`.

## running
There is only one mandatory argument, the path to the file containing the text
you want the algorithm to evolve towards. If not specified, the input will be
assumed to be stdin.

There are some sample texts in the [samples](./samples) folder for you to start
playing with.

## limitations
The most expensive operation is the distance computation between a chromosome's
solution and the goal text, which here is the hamming distance. In order to
speed this up, we use the SIMD implementation provided by [triple_accel][triple_accel],
which uses byte slices to represent strings.

This means **only ASCII text is supported by this crate**.

One possible way of solving this would be to use a fixed-length encoding for
unicode, like UTF-32, and very large SIMD vectors; alas there is no crate I'm
aware of that implements hamming distance like that, and building such is out of
scope for my weekend project.

[obermeyer-2009]: https://karlobermeyer.github.io/publications/obermeyer.ga_for_reconn_uav.2009.pdf
[triple_accel]: https://github.com/Daniel-Liu-c0deb0t/triple_accel
