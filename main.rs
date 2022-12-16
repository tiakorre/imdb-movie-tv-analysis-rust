use std::error::Error;
use std::fs;
use csv::Reader;
use petgraph::prelude::*;
use std::fs::File;
use std::io::{BufReader, BufRead};
use csv::StringRecord;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::HashMap;
use std::collections::hash_map::{Entry, OccupiedEntry};


fn main()  {
    let g = create_graph("imdb.csv");
    let num_edges = g.edge_count();
    println!("edges:{:?}", num_edges);
    let num_nodes = g.node_count();
    println!("nodes:{:?}", num_nodes);
    let degreecentrality = degree_centrality(&g);  
}
    
fn degree_centrality(graph: &Graph<(String,String), i32>)-> Vec<f64>{
    let mut map = HashMap::new();
    let mut final_vec = Vec::new();
    let num_nodes = graph.node_count();
    for node in graph.node_indices() {
        let label = graph.node_weight(node).unwrap();
        let num_connected_nodes = graph.neighbors(node).count();
        let mut dc_scaled = (num_connected_nodes as f64)/(num_nodes as f64);
        println!("Node:{:?} \n Label: {:?}\nNodes Connected to: {:?}\nDegree of Centrality:{:?}",node.index(),label,num_connected_nodes,dc_scaled);
        map.insert(node.index(), dc_scaled);
        final_vec.push(dc_scaled);
        
        

    }
    
    let max_entry = map.iter().fold(None, |acc, entry| {
        match acc {
            None => Some(entry),
            Some(acc_entry) => {
                if entry.1 > acc_entry.1 {
                    Some(entry)
                } else {
                    Some(acc_entry)
                }
            }
        }
    });
    
    if let Some(max_entry) = max_entry {
        println!("Node {} :is the most connected with a degree centrality score of {}", max_entry.0, max_entry.1);
    }
   
    
    
    let min_entry = map.iter().fold(None, |acc, entry| {
        match acc {
            None => Some(entry),
            Some(acc_entry) => {
                if entry.1 < acc_entry.1 {
                    Some(entry)
                } else {
                    Some(acc_entry)
                }
            }
        }
    });
    
    if let Some(min_entry) = min_entry {
        println!("Node {} :is the least connected with a degree centrality score of {}", min_entry.0, min_entry.1);
    }

    return final_vec;


    
   
}
    
fn create_graph(file_path: &str) -> Graph<(String,String), i32> {
    let mut graph = Graph::new();

    // Open the CSV file 
    let file = File::open(file_path).expect("Failed to open file");
    let mut reader = Reader::from_reader(BufReader::new(file));

    // CSV into a Vec
    let records = reader.records().collect::<Result<Vec<_>, _>>().expect("Failed to parse CSV file");

    // Add a nodes to the graph 
    for record in records {
        let node_value_dir = record[2].to_string(); 
        let node_value_met = record[1].to_string();
        graph.add_node((node_value_dir,node_value_met));
    }

    // create a loop that checks if two nodes have the same metascore value and if they do connect them. 
 
    for node1 in graph.node_indices() {
        for node2 in graph.node_indices() {
            // Skip the node if it's the same as the node1
            if node1 == node2 {
                continue;
            }
            // Get the second value of each node
            let value1 = &graph.node_weight(node1).unwrap().1;
            let value2 = &graph.node_weight(node2).unwrap().1;

            // Add an edge between the nodes if their second values are the same
            if value1 == value2 {
                let mut x = match value1.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => 0,};
                graph.add_edge(node1, node2, x);
            }     
        }
    }
    return graph;  
}



#[test]
fn test_add() {
    let mut graph = Graph::<(String,String), i32>::new();
    let mut a0 = String::from("Hello");
    let mut a1 = String::from("6");
    let mut b0 = String::from("Hello");
    let mut b1 = String::from("6");
    let mut c0 = String::from("Hello");
    let mut c1 = String::from("3");
    

    let a = graph.add_node((a0,a1));
    let b = graph.add_node((b0,b1));
    let c = graph.add_node((c0,c1));

    graph.add_edge(a, b, 6);

    assert_eq!(graph.node_count(), 3);
    assert_eq!(graph.edge_count(), 1);

    let vec1 = degree_centrality(&graph);
    let vec2 = Vec::from([0.33333, 0.0,0.0]);
    assert_eq!(vec1, vec2);

    
}
