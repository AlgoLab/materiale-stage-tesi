Una raccolta di materiale utile per tutti gli studenti che iniziano a lavorare

# Generale

*  [The Good Research Code Handbook](https://goodresearch.dev/) come organizzare un progetto

# Python

*  [Think Python](https://github.com/AllenDowney/ThinkPython2) anche [in Italiano](https://github.com/AllenDowney/ThinkPythonItalian)
*  [Biopython](https://biopython.org/)
*  [Python Speed](https://pythonspeed.com/) suggerimenti per velocizzare codice python

# Rust

## Materiale di riferimento

*  [Rust Book](https://doc.rust-lang.org/stable/book/) il libro ufficiale su Rust
*  [Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/readme.html) guida introduttiva
*  [Rust by Example](https://doc.rust-lang.org/rust-by-example/) diversi esempi introduttivi su come usare Rust
*  [Rustlings](https://github.com/rust-lang/rustlings) diversi esercizi introduttivi per imparare Rust (per ogni tipo di esercizio sono riportati i riferimenti al libro ufficiale)
*  [Nota su strutture dati in
   Rust](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/)
la gestione della memoria in Rust è diversa da quella di altri
linguaggi. Alcune strutture dati richiedono un'implmentazione peculiare
*  [Articolo su SIMD](https://neosmart.net/blog/2021/using-simd-acceleration-in-rust-to-create-the-worlds-fastest-tac/) come utilizzare SIMD in Rust
* [IDE e altri tools](https://www.rust-lang.org/tools) gli IDE consigliati sono **Clion** (unico con debugger integrato) e **VS Code** (ottimo per sviluppo remoto). Un tool molto utile è **Clippy**, che suggerisce versioni più idiomatiche del codice che scrivete, e soprattutto spiega ***il motivo*** per cui effettuare le modifiche
* [Rilevare memory leaks quando si usa unsafe](https://rustrepo.com/repo/japaric-rust-san-rust-testing) richiede compilatore nightly, se cargo è installato con [rustup](https://rustup.rs/) è possibile passare a nightly con `rustup default nightly` (per tornare alla versione "normale" basta usare `rustup default stable`)
* [Rust Performance Guide](https://nnethercote.github.io/perf-book/)

## Librerie suggerite
* [clap](https://crates.io/crates/clap) per realizzare interfacce a linea di comando
* [rayon](https://crates.io/crates/rayon) multi-threading
* [serde](https://crates.io/crates/serde) per serializzare/deserializzare in diversi formati
* [itertools](https://crates.io/crates/itertools)  diverse utilities
* [log](https://crates.io/crates/log) per effettuare logging
* [simple SDSL](https://github.com/jltsiren/simple-sds) strutture dati succinte
* [pyo3](https://crates.io/crates/pyo3) binding python-rust
* [niffler](https://github.com/luizirber/niffler/) per gestire file compressi
* [arewebioyet](https://rust4bio.github.io/arewebioyet) un altro elenco di risorse rust per la bioinformatica

Per maggiori dettagli su come utilizzare i Variation Graphs in Rust si faccia riferimento alla sottocartella **Rust - Variation Graphs**, che contiene un progetto d'esempio con la spiegazione di alcune funzionalità chiave.

# Read samples simulation
* Short genomic reads (Illumina):
   * https://github.com/nh13/DWGSIM
* RNA-Seq:
   * https://github.com/ldenti/RNASeqReadSimulator (simulatore naive)
   * https://confluence.sammeth.net/display/SIM/Home (simulatore che modella "RNA-Seq experiments in silico")
   * https://github.com/biomedbigdata/ASimulatoR (simulatore con supporto nativo per eventi di splicing alternativo)
* Long reads:
   * https://github.com/pfaucon/PBSIM-PacBio-Simulator
   * https://github.com/yukiteruono/pbsim2

# Introduction to Deep Learning
* Online courses
   * https://course.fast.ai/
* Youtube
   * [Alfredo Canziani](https://www.youtube.com/@alfcnz) (Profesor at New York University, course with Yann LeCun)
   * [CS231n Winter 2016, Fei-Fei Li & Andrej Karpathy & Justin Johnson](https://www.youtube.com/watch?v=NfnWJUyUJYU&list=PLkt2uSq6rBVctENoVBg1TpCC7OQi31AlC) Highly recommended to watch first 6 lessons.
* [Awesome-Deep-Learning: a curated list of books, courses, video lectures, and more](https://github.com/ChristosChristofidis/awesome-deep-learning)
* Papers:
   * [The NLP index](https://index.quantumstat.com/)
   * [paperwithcode.com](https://paperswithcode.com/)
* [CNN Explainer: Learn Convolutional Neural Network (CNN) in your browser!](https://poloclub.github.io/cnn-explainer/)
* Books:
   * [The little book of Deep Learning](https://fleuret.org/francois/lbdl.html)