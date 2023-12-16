use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{self, BufRead};
//create a struct GraphEdge that will keep track of source node and the target node of each subreddit name 

pub struct GraphEdge {
    source: String,
    target: String,
}

    // Read file for the graph
pub fn read_file(path: &str) -> Option<Vec<GraphEdge>> {
        
    let mut result: Vec<GraphEdge> = Vec::new();

    //mutable HashSet that stores unique tuples of (String,String) representing seen edges in the graph
    let mut seen_edges: HashSet<(String, String)> = HashSet::new();

    //this will attempt to open a finle using the provided path, if let contructs if the result is an Ok variant(file opening succeded)
    // a way if error handling
    if let Ok(file) = File::open(path) {
        let buf_reader = io::BufReader::new(file).lines();

        for line in buf_reader {
            if let Ok(line_str) = line {
                let v: Vec<&str> = line_str.trim().split('\t').collect();

                    // checks if the vector has at least two elements, this ensures it represents a valid edge in the graph
                if v.len() >= 2 {

                    // extracts the sourve and target substrings from the vector and converts them into its owned String instances
                    let source: String = v[0].to_string();
                    let target: String = v[1].to_string();

                    // checks if the tuple (source, target) is not already in the set and inserts this avoids duplicates and ensures unique edges 
                    if seen_edges.insert((source.clone(), target.clone())) {
                        // if the edge is unique, it creates a GraphEdge struct and adds it to the result vector 
                        result.push(GraphEdge { source, target });
                    }
                }

                // error handling 
            } else {
                eprintln!("Error reading line");
                return None;
            }
        }

        Some(result)
    } else {
        eprintln!("Could not open file");
        None
        }
    }

    // Create an adjacency list from graph edges that takes 
pub fn adjacency_list(graph_edges: &[GraphEdge]) -> HashMap<String, Vec<String>> {

    //first initiate and empty hashmap  the string as the key and a vector of strings as the neighboring vertices
    let mut adjacency_list: HashMap<String, Vec<String>> = HashMap::new();

    //iterates through each GraphEdge
    for connection in graph_edges {

        // extract refrences to the source and target verices from the current GraphEdge in the loop 
        let source: &String = &connection.source;
        let target: &String = &connection.target;

        //inserts an entry into the adjacency_list HashMap for the source vertex. If entry exists it appends the target vertex to the existing vector 
        // if it does not exist it creates a new entry with an empty vector and then appends the target vertex or vice-versa for the subsequent line
        adjacency_list.entry(source.clone()).or_insert(Vec::new()).push(target.clone());
        adjacency_list.entry(target.clone()).or_insert(Vec::new()).push(source.clone());
    }

    adjacency_list
}

// Compute the degree of separation between two nodes. graph a refrence to a HashMap where keys are strings and values are vectors of strings 
pub fn max_degree(graph: &HashMap<String, Vec<String>>, start_vertex: &str) {
    // initializes a mutable HashSet to keep track of the vertices that have been visited during the breadth-first-search
    let mut visited_vertices: HashSet<String> = HashSet::new();
    // stores vertices along with their degrees of separation, queue for bfs
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();

    // enqueues the start vertex with a degree of 0 into the queue
    queue.push_back((start_vertex.to_string(), 0));

    // marks the start vertex as visited by inserting it into the visited_vertices set
    visited_vertices.insert(start_vertex.to_string());

    // initialize max_degree before using it
    let mut max_degree = 0;

    // the while loop continues as long as the queue is not empty, dequeueing vertices along with their degrees
    while let Some((current_vertex, degree)) = queue.pop_front() {
        // update max_degree
        max_degree = max_degree.max(degree);

        // retrieves the neighbors of the current vertex from the graph
        if let Some(neighbors) = graph.get(&current_vertex) {
            for neighbor in neighbors {
                // checks if the neighbor has not been visited
                if !visited_vertices.contains(neighbor) {
                    // enqueues the neighbors with an incremented degree into the queue
                    queue.push_back((neighbor.clone(), degree + 1));
                    // marks neighbors as visited
                    visited_vertices.insert(neighbor.clone());
                }
            }
        }
    }

    // calculating max degree
    if max_degree > 0 {
        println!("Maximum degree of separation for {}: {}", start_vertex, max_degree);
    } else {
        println!("No path found for {}", start_vertex);
    }
}


// Compute and return the average degree of separation for a given node
pub fn average_degree(graph: &HashMap<String, Vec<String>>, start_vertex: &str) -> f64 {
    //intializes a mutable VecDeque name queue to store vertices along with their degrees of separation
    let mut visited_vertices: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();

    //initialize a mutable variable total degree to keep track of the sum of degrees during the search, same for total nodes 
    let mut total_degree: usize = 0;
    let mut total_nodes: usize = 0;

    // the starting vertex is added to the queue with a degree of 0, and the starting vertex is marked as visited
    queue.push_back((start_vertex.to_string(), 0));
    visited_vertices.insert(start_vertex.to_string());
    // while loop starts that countinues until the quenue is empty. extract current vertex and its degree from the front of the queue like a line 
    while let Some((current_vertex, degree)) = queue.pop_front() {
        total_degree += degree;
        total_nodes += 1; //updates the total degree and total nodes with the current vertex's degree 

        // checks if the current vertex has neighbors in the graph 
        if let Some(neighbors) = graph.get(&current_vertex) {
            // iterates through the neighbors of the current vertex 
            for neighbor in neighbors {
                if !visited_vertices.contains(neighbor) {
                    // Explore neighbors
                    queue.push_back((neighbor.clone(), degree + 1));
                    visited_vertices.insert(neighbor.clone());
                }
            }
        }
    }
    // calculates and returns the average degree of separation as a f64
    if total_nodes > 0 {
        total_degree as f64 / total_nodes as f64
    } else {
        0.0
    }
}


