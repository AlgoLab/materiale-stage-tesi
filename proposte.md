# Proposte di Stage (Laurea Triennale in Informatica)

## Confronto di indici per analisi pangenomiche

L'obiettivo dello stage è quello di confrontare l'indice [ropebwt2](https://github.com/lh3/ropebwt2) con il più recente indice [movi](https://github.com/mohsenzakeri/movi). Lo studente deve saper programmare in C++ e avere confidenza con i formati FASTA e FASTQ.

## Identificazione di eventi di splicing utilizzando algoritmi di flusso su pantrascrittomi
L'obiettivo dello stage è quello di estendere pantas ed esplorare la chiamata di eventi di splicing utilizzando grafi del pantrascrittoma locali ai geni e un algoritmo di flusso per l'identificazione dei potenziali trascritti coinvolti nell'evento. Lo studente deve saper programmare in python e avere confidenza con il formato gfa (formato gaf è un plus).

##  Scalable Visualization of k-mer distributions with the Chaos Game Representation of DNA

This project aims to visualize the k-mer distribution of sequences and samples of reads by leveraging the Chaos Game Representation of DNA (CGR) — this encoding orders k-mers in a square and its frequencies are used as colors in the image. A similar encoding was recently developed, named  ComplexCGR, with the difference that k-mers are assigned a polar coordinate in a circumference.
The student will be in charge of generating these visualizations to use the largest k possible (k<31) with both CGR and ComplexCGR.

**Requirements:** Familiarity with fasta and fastq formats. Programming in one of these languages C, C++, or Rust.

## Classification of genomic sequences with metric learning 
In a classification problem, instances (genomic sequences) are associated with a label. From a supervised learning point of view, the goal is to assign a label to an input sequence by learning a probability distribution among the universe of labels, while from a metric learning point of view, the goal is to learn vector representations (embeddings) for each input, such that closer vector representations share the same label.
In this project, the student will classify Sars-CoV-2 sequences into clades by using metric learning, comparing two loss functions: contrastive-loss and triplet-loss, and benchmark the result against a supervised approach.

**Requirements:** Familiarity with fasta format. Proficiency in Python. Basic knowledge of deep learning, and familiarity with TensorFlow or Pytorch.


## Proposte di stage sulla gestione di formati di file

1. `rs-gfa`: libreria Rust per la gestione di file [GFA](https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md).
Il parser dovrebbe usare Pest e essere basato su una PEG. Bisogna realizzare dei binding Python, analogamente a quanto già fatto per rs-gax.
3. `gfatk`: programma CLI per la gestione di file GFA, simile a seqtk. Ad esempio, devono essere possibili estrazioni solo di una parte di grafo, selezione di porzioni o cammini, dump in formato testuale, json, sqlite. Inizialmente in python per un prototipo, poi migrato a Rust. Usa la libreria `pygfa` e/o `rsgfa`. Il progetto https://github.com/tolkit/gfatk non è stato aggiornato negli ultimi mesi e sembra essere disegnato per grafi piccoli. Ha il pregio di essere basato su petgraph.
Il primo prototipo può essere in Python, ma sarebbe gradita un'implementazione in Rust.
4. [rs-gax](https://github.com/AlgoLab/rs-gax): libreria Rust per la gestione di file [GAF](https://github.com/lh3/gfatools/blob/master/doc/rGFA.md#the-graph-alignment-format-gaf), [GAM](https://github.com/vgteam/vg/wiki/File-Formats#gam-graph-alignment--map-vgs-bam), GAMP. Attualmente usa https://github.com/chfi/rs-gfa ma dovremmo valutare un passaggio ad un grafo che non sia quello di vg/handlegraph. Aggiungere la gestione del formato [GBZ](https://github.com/jltsiren/gbwtgraph/blob/master/SERIALIZATION.md). 
5. `gaxtk`: programma CLI per la gestione di file GAF, GAM, etc.
Il primo protitipo può essere in Python, ma sarebbe gradita un'implementazione in Rust.



# Proposte di tesi (Laurea Magistrale in Informatica)

1.  Formato [PGVF](https://github.com/pangenome/pgvf-spec). In questo caso bisogna fare un'analisi critica della specifica e progettare un formato binario, oltre a scrivere un parser e progettare una rappresentazione interna di un allineamento fra grafi
2.  Estendere [Handlegraph](https://pangenome.github.io/handlegraph/index.html) con una nuova rappresentazione interna compressa del grafo, ispirata da [WebGraph](https://webgraph.di.unimi.it/)
