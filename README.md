# Bioinformatics

This repository is my **personal learning lab for bioinformatics algorithms implemented in Rust**.

It contains:
- Core string and sequence algorithms
- Pattern matching and motif search
- DNA/RNA sequence processing
- Algorithmic biology foundations
- Performance-oriented Rust implementations

This repo will **grow continuously** as I progress through bioinformatics concepts and Rust mastery.

## Purpose

The goals of this repository are to:

- Learn **bioinformatics from first principles**
- Practice **real algorithmic implementations in Rust**
- Build a **public technical learning trail**
- Develop **low-level performance intuition**
- Prepare for **advanced domains (genomics, crypto-biology, systems + biology intersections)**

This is not a tutorial repo — it’s a **working engineering notebook**.

## 🧠 Topics Covered (Evolving)

- ✅ Pattern counting (sliding window)
- ⏳ k-mers & frequency maps
- ⏳ Reverse complements
- ⏳ Hamming distance
- ⏳ Motif finding
- ⏳ Approximate pattern matching
- ⏳ Genome replication algorithms
- ⏳ Sequence alignment (later)

This list will grow step by step.

## Tech Stack

- **Language:** Rust
- **Build System:** Cargo
- **Focus:** Safety, performance, correctness
- **Approach:** Low-level byte processing where relevant

## Running the Code

Clone the repository:

```bash
git clone https://github.com/vinecksie/biotech.git
cd bioinformatics-rust
```

## Run the current main program:

```bash
cargo run
```

## Run tests (as they get added):

```bash
cargo test
```

## 📂 Project Structure (Will Evolve)
```css
.
├── src/
│   ├── main.rs
│   ├── pattern_count.rs
│   ├── kmers.rs
│   ├── motif.rs
│   └── ...
├── Cargo.toml
└── README.md
```

Each file represents a standalone algorithm or concept.

## Philosophy
- No black-box libraries for core algorithms
- Everything is implemented from scratch
- Correctness first, then performance
- Each algorithm is expected to be:
- Testable
- Understandable
- Re-usable

## Roadmap (Open)
- Complete basic string algorithms
- Add unit tests for each algorithm
- Add benchmarks
- Add FASTA/FASTQ parsing
- Add simple alignment algorithms
- Memory-optimized implementations

## License
MIT License — free to use, study, and modify.
