mod graph;

use crate::graph::{read_file, adjacency_list, max_degree, average_degree};

fn main() {
    // declares file path
    let file_path: &str = "soc-redditHyperlinks-body.tsv";

    // reads contents of the file
    if let Some(subreddit_connections) = read_file(file_path) {
        // creates an adjacency list from the data in the subreddit_connections 
        let adjacency_list = adjacency_list(&subreddit_connections);
         // starts a loop that iterates over the first 10 keys(subreddits) in the adjacency list
        for node in adjacency_list.keys().take(10) {
            println!("Node: {}", node);
            // Compute and prints average degree of separation for the current node using the average degree of separation function.
            let avg_degree = average_degree(&adjacency_list, node);
            println!("Average Degree of Separation: {:.2}", avg_degree);

            // Use the same node for specific degree of separation
            max_degree(&adjacency_list, node);

            println!("---------------------");
        }
    }
}



