# HGS-CVRP in Rust

A Rust port of the [HGS-CVRP](https://github.com/vidalt/HGS-CVRP) reference implementation:
the Hybrid Genetic Search for the Capacitated Vehicle Routing Problem with the SWAP*
neighborhood (Vidal 2022, MIT license).

The port is faithful: every algorithmic component (Split, granular RI local search,
SWAP*, population/diversity management, adaptive penalties) follows the C++ code
line by line, so results and performance are directly comparable.

## Building and running

```console
cargo build --release
./target/release/hgs ../Instances/CVRP/X-n157-k13.vrp mySolution.sol -seed 1 -t 30
```

The command line interface is identical to the C++ executable (same flags, same
defaults, same log and solution file formats). Run without arguments to print the help.

Run the unit tests with `cargo test`.

## Library usage

The crate also exposes the solver as a library (see the example in `src/lib.rs`):
read or build an instance, construct `Params`, then run `Genetic`. This plays the
role of the C interface in the original project.

## Code structure

The modules map one-to-one to the C++ source files:

| Rust module               | C++ file              | Content                                            |
|---------------------------|-----------------------|----------------------------------------------------|
| `params`                  | `Params.*`            | Instance data, penalties, RNG, correlated vertices |
| `individual`              | `Individual.*`        | Solution representation, evaluation, solution I/O  |
| `population`              | `Population.*`        | Subpopulations, diversity, penalties, best tracking|
| `genetic`                 | `Genetic.*`           | Main GA loop and OX crossover                      |
| `local_search`            | `LocalSearch.*`       | RI moves (1-9) and SWAP* neighborhood              |
| `split`                   | `Split.*`             | Linear Split (limited and unlimited fleet)         |
| `circle_sector`           | `CircleSector.h`      | Circle sectors for SWAP* pruning                   |
| `algorithm_parameters`    | `AlgorithmParameters.*` | HGS parameters and defaults                      |
| `cvrplib`                 | `InstanceCVRPLIB.*`   | CVRPLIB instance reader                            |
| `cli`                     | `commandline.h`       | Command line parsing                               |
| `rng`, `matrix`, `util`   | -                     | minstd LCG, flat distance matrix, %g formatting    |

Design notes (differences imposed or encouraged by Rust):

- **No pointers, no reference counting.** The linked list of the local search lives in
  a single `Vec<Node>` arena and links are indices; population members are identified
  by ids instead of addresses. Updates remain O(1) and there is no `Rc`/`RefCell`/unsafe.
- **Mutable state is threaded explicitly.** The C++ classes all hold `Params&` and
  mutate the penalties/RNG through it; here a `&mut Params` is passed to the calls
  that need it, which makes the data flow visible and keeps the borrow checker happy.
- **The only solution copies are the semantically required ones** (storing an
  individual into the population, tracking the best solution; the latter uses
  `clone_from` to recycle buffers).

## Behavioral parity with the C++ implementation

Verified on the same test set as the upstream CI (seed 1, default termination):

| Instance     | Options    | C++ cost | Rust cost |
|--------------|------------|----------|-----------|
| X-n101-k25   | `-round 1` | 27591    | 27591     |
| X-n110-k13   | `-round 1` | 14971    | 14971     |
| CMT6         | `-round 0` | 555.43   | 555.43    |
| CMT7         | `-round 0` | 909.675  | 909.675   |

Time per iteration is on par with the C++ build (within a few percent on X-n502-k39).

Known, intentional differences:

- **Random streams.** The RNG engine is the same `minstd_rand` LCG, but `shuffle` and
  the uniform distributions are own implementations (the C++ standard leaves them
  implementation-defined), so the sequence of visited solutions differs from a given
  libstdc++ build. Solution quality is statistically equivalent.
- **Clock.** Times are measured with a monotonic wall clock instead of `clock()`
  (CPU time); both coincide for this single-threaded program.
- **Errors.** User-facing errors (bad command line, unreadable instance) are reported
  as `EXCEPTION | ...` like the C++ version; internal invariant violations
  (e.g. a failed Split propagation) panic instead of throwing.
- Invalid numeric command-line values are rejected with an error message
  (C++ `atoi` would silently read 0).
