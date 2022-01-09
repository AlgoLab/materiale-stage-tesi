Una raccolta di materiale utile per tutti gli studenti che iniziano a lavorare

# Generale

*  [The Good Research Code Handbook](https://goodresearch.dev/) come organizzare un progetto

# Python

*  [Biopython](https://biopython.org/)

# Rust

## Materiale di riferimento

*  [Rust Book](https://doc.rust-lang.org/stable/book/) il libro ufficiale su Rust
*  [Gentle Intro](https://stevedonovan.github.io/rust-gentle-intro/readme.html) guida introduttiva
*  [Rust by Example](https://doc.rust-lang.org/rust-by-example/) diversi esempi introduttivi su come usare Rust
*  [Nota su strutture dati in
   Rust](https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/)
la gestione della memoria in Rust è diversa da quella di altri
linguaggi. Alcune strutture dati richiedono un'implmentazione peculiare.
*  [Articolo su SIMD](https://neosmart.net/blog/2021/using-simd-acceleration-in-rust-to-create-the-worlds-fastest-tac/) come utilizzare SIMD in Rust
* [IDE e altri tools](https://www.rust-lang.org/tools) gli IDE consigliati sono **Clion** (unico con debugger integrato) e **VS Code** (ottimo per sviluppo remoto). Un tool molto utile è **Clippy**, che suggerisce versioni più idiomatiche del codice che scrivete, e soprattutto spiega ***il motivo*** per cui effettuare le modifiche.

## Librerie suggerite
* [clap](https://crates.io/crates/clap) per realizzare interfacce a linea di comando
* [rayon](https://crates.io/crates/rayon) multi-threading
* [serde](https://crates.io/crates/serde) per serializzare/deserializzare in diversi formati
* [itertools](https://crates.io/crates/itertools)  diverse utilities
* [log](https://crates.io/crates/log) per effettuare logging
* [simple SDSL](https://github.com/jltsiren/simple-sds) strutture dati succinte
*  [pyo3](https://crates.io/crates/pyo3) binding python-rust

Per maggiori dettagli su come utilizzare i Variation Graphs in Rust si faccia riferimento alla sottocartella **Rust - Variation Graphs**, che contiene un progetto d'esempio con la spiegazione di alcune funzionalità chiave.
