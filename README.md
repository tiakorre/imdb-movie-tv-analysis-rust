# IMDb Movie & TV Graph Analysis in Rust


This project demonstrates an **end-to-end exploration** of movie relationships using **graph theory** in Rust. It highlights how **degree centrality** can reveal the most and least connected entities, provides insights about common metascores, and serves as a template for analyzing networks in other datasets.

## Outputs

- The program outputs information about the number of nodes and edges in the graph.
- It identifies and displays the most and least connected nodes, including their degree centrality scores.
---

## Dataset

The dataset used is from Kaggle and contains the **top 1000 movies and TV shows on IMDb**, with 16 columns including:

- Poster link  
- Name of the movie  
- Year of release  
- Certificates  
- Runtime  
- Genre  
- IMDb rating  
- Summary  
- Metascore  
- Director  
- Number of votes  
- Box office earnings  
- Three columns for star actors  

There are around 1000 entries spanning a wide range of years.  

**Features used for analysis**:  
- Director  
- Metascore  

The graph connects nodes (director + metascore) based on **matching metascores**.



## Features

- **Graph-based analysis**: Constructs a graph from the CSV file to analyze relationships between movies and TV shows.  
- **Degree centrality calculations**: Computes degree centrality to identify the most and least connected nodes.  
- **Node labeling**: Each node includes the director and metascore of a movie.  
- **Edge creation**: Nodes are connected if they share the same metascore.  



## Key Insights

- Movies with a **Metascore of 76** are the most common and highly connected in the network.  
- Movies with a **Metascore of 40** are the least common and least connected.  
- **Most connected node**: Node 588 → Director: Jim Sheridan, Metascore: 76, Centrality score: 0.03924  
- **Least connected node**: Node 539 → Director: Abhishek Kapoor, Metascore: 40, Centrality score: 0  
- Many movies share the same metascore, meaning the maximum and minimum centrality nodes can change even if the scores remain the same.  
- The analysis highlights how directors and their movies cluster around common metascores, showing trends in movie ratings across the dataset.  



___
## To run the analysis locally, follow these steps:

1. **Clone the repository**:
    ```bash
    git clone https://github.com/tiakorre/imdb-movie-tv-anaysis-rust.git
    ```
2. **Navigate to the project directory**:
    ```bash
    cd imdb-movie-tv-anaysis-rust
    ```
3. **Compile and build the project using Cargo**:
    ```bash
    cargo build
    ```

4. **Run the program**:
    ```bash
    cargo run
    ```



 
