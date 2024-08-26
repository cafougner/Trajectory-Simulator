<h1 align = "center">Trajectory Simulator</h1>
<p align = "center">A program for simulating and analyzing projectile trajectories for FRC.</p>

## Install
To use the program, you need to have both [Rust](https://www.rust-lag.org/tools/install) (1.80.1 as of writing) and [Python](https://www.python.org/downloads/) (3.12.5 as of writing) installed.

You can install the Python dependencies (numpy, pandas, and scikit-learn) by running

```sh
$ python3 -m pip install -r requirements.txt
```

## Usage
You can compile the program and run the current simulation configuration by running

```sh
$ cargo run --release
```

and then a polynomial can be fit to the simulation results by running

```sh
$ python3 -m results_equation_generator.py
```

which will output an `equation.txt` file to the `results` folder, containing the polynomial equation, along with its degree and max/mean errors.

## Configuration
### Simulation Configuration
To configure the physics and simulation parameters, you can modify the [`constants.rs`](./src/constants.rs) file.

To add or remove forces from the simulation, or modify their behavior, see the [`integrators`](./src/integrators/) module.

### Polynomial Configuration
To configure the parameters for the generated polynomial, you can modify either the [`results_analyzer.py`](./results_analyzer.py) script, or the [`results_equation_generator.py`](./results_equation_generator.py) script. If you modify one of them, you will need to update the other to match.

## Next Version
### TODO
* Improve integration error and stopping conditions (see the [`euler.rs`](./src/integrators/euler.rs) TODO comment).

* Support obstacles / indirect trajectories (see the [`euler.rs`](./src/integrators/euler.rs) TODO comment).

* Finish the [`results_analyzer.py`](./results_analyzer.py) script.

* Implement multithreaded CPU and GPU simulations.

<!-- * Nothing! It's *perfect*. -->

### Known Bugs
* None! The code is *flawless*.
