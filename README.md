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

## 🧪 Using PyAlgebraRust from Source

PyAlgebraRust can be used and inspected directly from source. This is useful for
research, experimentation, or contributing to the Rust implementation.

Installs the Rust extension into a local Python virtual environment.

#### Requirements
- Python ≥ 3.9
- Rust toolchain (stable)
- `maturin`

#### Steps

```bash
git clone https://github.com/mehrdadmaleki-labs/PyAlgebraRust.git
cd PyAlgebraRust

python -m venv .venv
source .venv/bin/activate

pip install maturin
maturin develop --release


## 🧑‍💻 Example Usage

```python
import pyalgebrarust as alg

# Computed fully in Rust using async Tokio and parallel execution
x = alg.factorial_parallel(300)
print(x)
<details> <summary><strong>306057512216440636035370461297268629388588804173576999416776741259476533176716867465515291422477573349939147888701726368864263907759003154226842927906974559841225476930271954604008012215776252176854255965356903506788725264321896264299365204576448830388909753943489625436053225980776521270822437639449120128678675368305712293681943649956460498166450227716500185176546469340112226034729724066333258583506870150169794168850353752137554910289126407157154830282284937952636580145235233156936482233436799254594095276820608062232812387383880817049600000000000000000000000000000000000000000000000000000000000000000000000000
</strong></summary>
