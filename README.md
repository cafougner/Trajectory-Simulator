<h1 align = "center">Trajectory Solver</h1>
<p align = "center">A program for simulating and analyzing projectile trajectories for FRC.</p>

## Install
To configure and run simulations, you need to install [Rust](https://www.rust-lang.org/tools/install) (1.80.1 as of writing).

To analyze the simulation results, you need to install [Python](https://www.python.org/downloads/) (3.12.5 as of writing).

###
###
For Python, you will also need to run:
```sh
$ pip install -r requirements.txt
```

## Usage
To configure the simulation parameters, you can modify the [`constants.rs`](src/constants.rs).

To add/remove forces from the simulation, see the [`integrators`](src/integrators) module.

###
###
To compile the program and run the simulation, run:
```sh
$ cargo run --release
```

To generate a polynomial from the `results.csv`, run:
```sh
$ python results_equation_generator.py
```
which will output an `equation.txt` file into the `results` folder, containing the polynomial equation, degree, and the max and mean errors.

###
###
The parameters for the polynomial can be configured by editing the [`results_equation_generator.py`](results_equation_generator.py) script.

## Next Version
### TODO:
* Finish [`results_analyzer.py`](results_analyzer.py).

* Implement multithreaded CPU and GPU simulations.

* Improve integration error and stopping conditions (see [`euler.rs`](src/integrators/euler.rs) TODO).

* Support indirect trajectories and obstacles (see [`euler.rs`](src/integrators/euler.rs) TODO).

<!-- * Nothing! It's *perfect*. -->

### Known Bugs:
* None! The code is *flawless*.
