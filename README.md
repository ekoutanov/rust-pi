`rust-pi`
===
Derivation of Pi using elemental Monte-Carlo techniques (sampling in the Cartesian plane within the unit radius), and
subsequent estimation of its confidence interval using the Dvoretzky–Kiefer–Wolfowitz inequality.

To run (assuming availability of `just` and `cargo`):

```sh
just run
```

Expect output like:

```
n: 1000, trials: 1000000, took 0.8 s
min: 3.136740000, median: 3.141608000, max: 3.147012000
mean: 3.141579912
normal:
  CI (α=0.1):  [3.141496558, 3.141663266], x̄ = 3.141579912 ± 0.000083354
  CI (α=0.05): [3.141480590, 3.141679234], x̄ = 3.141579912 ± 0.000099322
unadjusted nonparametric:
  CI (α=0.1):  [3.138924000, 3.144168000], x̄ = 3.141546000 ± 0.002622000
  CI (α=0.05): [3.138344000, 3.144560000], x̄ = 3.141452000 ± 0.003108000
Dvoretzky–Kiefer–Wolfowitz nonparametric:
  CI (α=0.1):  [3.141263907, 3.141927113], x̄ = 3.141595510 ± 0.000331603
  CI (α=0.05): [3.141231887, 3.141962535], x̄ = 3.141597211 ± 0.000365324
reference Pi: 3.141592653589793
```