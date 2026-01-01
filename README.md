# PyAlgebraRust 🦀🐍

**High-performance algebra and numerical computing for Python, powered by Rust.**

PyAlgebraRust is a Python library implemented in Rust that provides fast, safe, and scalable tools for linear algebra, extended number systems, and advanced mathematical functions. Performance-critical computations are implemented in Rust and exposed through a clean Python API.

---

## ✨ Key Features

- 🚀 **Rust-powered performance**  
  Core numerical routines are written in Rust for speed, memory safety, and concurrency.

- 🧮 **Mathematical Functions**
  - Factorial and combinatorial functions
  - Large-number arithmetic without fixed-size integer overflow
  - Parallelized numeric reductions

- 🔄 **Async & Parallel Execution**
  - Mathematical functions (e.g. factorial) implemented using **Tokio async runtime**
  - Workload automatically split into independent chunks
  - CPU-heavy tasks executed in parallel using Rust thread pools
  - Python GIL released during computation

- 🔢 **Extended Number Systems**
  - Complex numbers
  - Dual numbers (for automatic differentiation)
  - Foundations for higher-order numeric types

- 🧮 **Linear Algebra**
  - Vectors and matrices
  - Matrix operations and products
  - Parallel numerical kernels (planned)

- 🐍 **Pythonic API**
  - Simple, expressive Python interface
  - All heavy computation performed in Rust

---

---

## 🧑‍💻 Example Usage

```python
import pyalgebrarust as alg

# Computed fully in Rust using async Tokio and parallel execution
x = alg.factorial_parallel(300)
print(x)
