// Handlegraph è un'interfaccia, definisce una serie di metodi e struct;
// Hashgraph è invece l'implementazione vera e propria, ed è basata su tabelle di hash per
// memorizzare nodi e archi. L'idea è che (in futuro) vengano create nuove implementazioni,
// ed utilizzando un'interfaccia condivisa passare da una all'altra non dovrebbe essere difficile.
use handlegraph::{
    handle::{Direction, Edge, Handle, NodeId},
    handlegraph::HandleGraph,
    hashgraph::{HashGraph, PathStep},
    mutablehandlegraph::MutableHandleGraph,
    pathgraph::PathHandleGraph,
};

// Queste dipendenze servono per parsare un grafo in formato GFA.
// Link alla specifica del formato: https://gfa-spec.github.io/GFA-spec/GFA1.html
use gfa::{
    parser::GFAParser,
    gfa::*,
};

// Richiesto per convertire byte in stringhe (si può ignorare ai fini della spiegazione)
use bstr::ByteVec;

fn main() {

    //----------------------------------------------------------
    // Esempio 1: parsare un file GFA
    // Di solito si lavora su grafi già esistenti, memorizzati in formato GFA.

    // Creo il parser
    let parser = GFAParser::new();

    // Faccio il parsing del GFA
    let gfa: GFA<usize, ()> =
            parser.parse_file(&"./input/prova.gfa").unwrap();

    // Creo il grafo a partire dal gfa
    let _graph: HashGraph = HashGraph::from_gfa(&gfa);
    //--------------------------------------------------------


    //----------------------------------------------------------
    // Esempio 2: creare un grafo del pangenoma da zero, e aggiungere un nodo

    let mut graph2: HashGraph = HashGraph::new();

    // Creo un nodo con la sequenza "AAATC"
    // Nota: La sequenza è da passare come bytes
    let h1 = graph2.append_handle("AAATC".as_bytes());

    // Questo metodo ritorna un handle e non un nodo vero e proprio.
    // Un handle è un nodo "orientato": visto che il Variation Graph rappresenta
    // entrambi gli strand di DNA, è (quasi sempre) necessario avere un riferimento del tipo
    // (id_nodo, orientamento), ovvero un handle.
    println!("E' stato creato l'handle {}, corrispondente al nodo {}", h1.as_integer(), h1.id());
    println!("La sequenza associata all'handle è: {}", graph2.sequence(h1).into_string().unwrap());

    // L'handle creato è di default Forward
    assert_eq!(h1.is_reverse(), false);

    // E' possibile "flippare" l'handle per ottenere quello sull'orientazione opposta
    let flipped = h1.flip();
    println!("L'handle flippato è {}, corrispondente al nodo {}", flipped.as_integer(), flipped.id());
    println!("La sequenza associata all'handle flippato è: {}", graph2.sequence(flipped).into_string().unwrap());

    // Nota importante: nel grafo c'è comunque un solo nodo!
    assert_eq!(graph2.node_count(), 1);
    // e ogni nodo può essere visto sia lato forward che lato reverse, che corrisponde ai due handle.
    // In generale si ha che gli handle pari sono forward, e quelli reverse sono dispari.

    // ----------------------------------------------------------


    // ----------------------------------------------------------
    // Esempio 3: aggiungere nodi, archi e path al grafo precedente

    //Creo altri nodi (o meglio, handle)
    let h2 = graph2.append_handle("T".as_bytes());
    let h3 = graph2.append_handle("CG".as_bytes());
    let h4 = graph2.append_handle("AA".as_bytes());

    // Creo archi tra nodi
    graph2.create_edge(&Edge(h1,h2));
    graph2.create_edge(&Edge(h1,h3));
    graph2.create_edge(&Edge(h2,h4));
    graph2.create_edge(&Edge(h3,h4));

    // Creo un nuovo path
    let p1 = graph2.create_path_handle("P1".as_bytes(), false);
    graph2.append_step(&p1, h1);
    graph2.append_step(&p1, h2);
    graph2.append_step(&p1, h4);

    // In generale queste operazioni (aggiunta nodi, archi, path) non sono da eseguire manualmente,
    // ma vengono create in automatico quando si parsa un GFA. Tuttavia possono sempre tornare
    // utili per testing/debugging ecc.
    //--------------------------------------------------------


    //----------------------------------------------------------
    // Esempio 3: navigare il grafo

    // Creo un iteratore sugli handles/nodi del grafo
    println!("Handle disordinati:");
    for handle in graph2.handles_iter() {
        println!("Handle {}", handle.as_integer());
    }

    // Nota: non sono necessariamente in ordine. Per avere gli handle ordinati si può collezionare
    // l'iteratore in un vettore e sortare il vettore.
    println!("Handle ordinati:");
    let mut sorted_handles: Vec<Handle> = graph2.handles_iter().collect();
    sorted_handles.sort();
    for handle in sorted_handles {
        println!("Handle {}", handle.as_integer());
    }

    // Ottengo gli handle immediatamente a destra di h1
    println!("Vicini di h1 (a destra):");
    for neighbour in graph2.handle_edges_iter(h1, Direction::Right) {
        println!("Handle {}",neighbour.as_integer());
    }
    // A sinistra non c'è nulla!
    assert_eq!(graph2.handle_edges_iter(h1, Direction::Left).count(), 0);

    // Nota: se l'handle di partenza è reverse, la sinistra e la destra risultano invertiti.
    // (visto che lo strand è quello opposto, è come guardare il grafo "al contrario")
    let h1_flipped = h1.flip();
    assert_eq!(graph2.handle_edges_iter(h1_flipped, Direction::Right).count(), 0);

    println!("Vicini di h1_flipped (a sinistra):");
    for neighbour in graph2.handle_edges_iter(h1_flipped, Direction::Left) {
        println!("Handle {}", neighbour.as_integer());
    }
    // Nota: Gli iteratori lavorano su handle, quindi bisogna sempre tenere conto dell'orientazione.

    //--------------------------------------------------------

}
