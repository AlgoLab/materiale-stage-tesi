# Esempi di utilizzo di Handlegraph in Rust

Questo è un progetto d'esempio che spiega come utilizzare Handlegraph per lavorare con i Variation Graphs in Rust. Per visualizzare l'output è necessario muoversi su questa cartella e lanciare il comando:

```
cargo run
```

Il codice (che si trova in **src/main.rs**) è strutturato in una serie di esempi che spiegano le principali funzioni per ***creare***, ***modificare*** e ***navigare*** il grafo. 

Suggerisco di utilizzare l'autocomplete (disponibile ad esempio in Clion) per provare tutte le funzioni disponibili. In alternativa, si può fare riferimento alla documentazione delle due librerie.


## Librerie per Variation Graphs
- [handlegraph](https://crates.io/crates/handlegraph) gestione dei Variation Graphs in rust
- [gfa](https://crates.io/crates/gfa) parsing di file in formato GFA

## I nostri progetti
- [rs-gfatovcf](https://github.com/HopedWall/rs-gfatovcf) variant calling
- [rs-vgaligner](https://github.com/AlgoLab/rs-vgaligner) allineamento read-grafo
- [rs-abPOA](https://github.com/HopedWall/rs-abPOA) bindings Rust per abPOA

## Papers di riferimento e tools disponibili
- [Computational pan-genomics: status, promises and challenges (The Computational Pan-Genomics Consortium)](https://pubmed.ncbi.nlm.nih.gov/27769991/)
- [Pangenome Graphs (Eizenga et al.)](https://pubmed.ncbi.nlm.nih.gov/32453966/)
- [Genome graphs and the evolution of genome inference (Paten et al.)](https://pubmed.ncbi.nlm.nih.gov/28360232/)
- [Variation graph toolkit improves read mapping by representing genetic variation in the reference (Garrison et al.)](https://www.nature.com/articles/nbt.4227)


Per una panoramica dei tool che utilizzano i Variation Graphs, si può fare riferimento a [questo indirizzo](https://pangenome.github.io/).
