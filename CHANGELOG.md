# Changelog

Integraal aims to provide generic and efficient tools for numerical integration in the Rust Programming Language.

---

## 0.1.0

Integraal's first (real) release is here!

The crate features a main structure, `Integraal`, that acts as the entrypoint for your numerical integration.
You can specify the domain and the function of the integral, as well as the numerical integration method using
methods of the structure. It follows a builder-like pattern.

Domains and functions are described using struct enums for the sake of flexibility. The repository contains
usage examples for both analytical expression and sampled values integration. The following numerical
integration methods are implemented:

- Riemann summation (both left rule and right rule)
- Trapezoid rule
- Simpson's rule (different variants depending on the domain & function)
- Boole's rule
- Romberg's rule
- Monte Carlo method

Different methods have different requirements and dependencies. All of this is detailed in the documentation.

In the future, the computation kernel may support different execution backends. This would align with the initial
motivation behind this crate, that is being an experiment over flexible API designs for HPC.

