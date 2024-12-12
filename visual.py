import matplotlib.pyplot as plt
import networkx as nx
import csv
import glob

# Load graph data from the CSV file
def load_graph_from_csv(file_path):
    graph = nx.Graph()
    with open(file_path, 'r') as csvfile:
        reader = csv.reader(csvfile)
        next(reader)  # Skip the header
        unique_nodes = []
        for i, row in enumerate(reader):
            try:
                node1, node2, weight = row[0], row[1], float(row[2])
                if node1 not in unique_nodes:
                    unique_nodes.append(node1)
                if node2 not in unique_nodes:
                    unique_nodes.append(node2)
                graph.add_edge(node1, node2, weight=weight)
            except Exception as e:
                print(f"Error on row {i}: {row} - {e}")
    if len(unique_nodes) < 15:
        for node in unique_nodes:
            print(f"Here is a unique node: {node}")
    print(f"Loaded graph with {len(graph.nodes)} nodes and {len(graph.edges)} edges.")
    return graph


# Function to draw and export a filtered graph with a specific edge color
def export_filtered_graph(graph, pos, edges, output_file):
    """Export a graph with filtered edges and a uniform edge color."""
    plt.figure(figsize=(16, 9), dpi=300)  # High-resolution settings
    nx.draw_networkx_nodes(graph, pos, node_size=50, node_color="lightblue")
    nx.draw_networkx_edges(
        graph, pos, edgelist=edges, edge_color="gray", alpha=0.7, width=1.0
    )
    nx.draw_networkx_labels(graph, pos, font_size=5, font_color="black")
    plt.axis("off")
    plt.savefig(output_file, dpi=300, bbox_inches="tight", pad_inches=0)
    plt.close()
    print(f"Graph saved as {output_file}")

# Main function to process and export graphs
def export_graphs_from_files(csv_files, output_prefix="beta_graph"):
    """Load and export graphs from a list of CSV files."""
    for i, file_path in enumerate(csv_files):
        graph = load_graph_from_csv(file_path)
        pos = nx.spring_layout(graph, seed=42)  # Consistent layout
        edges = list(graph.edges(data=True))
        output_file = f"{output_prefix}_{i + 1}_gray.png"
        print(f"Graph {i + 1}: {len(graph.nodes)} nodes, {len(graph.edges)} edges")
        export_filtered_graph(graph, pos, edges, output_file)

# Load and export the graphs
if __name__ == "__main__":
    csv_files = glob.glob("graph_*.csv")  # Assuming files are named graph_1.csv, graph_2.csv, etc.
    export_graphs_from_files(csv_files)
