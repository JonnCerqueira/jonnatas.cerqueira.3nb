use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::Dfs;

pub struct MyGraph {
    graph: Graph<&'static str, ()>,
}

impl MyGraph {
    pub fn new() -> Self {
        let mut graph = Graph::new();
        
        // Adiciona nós com rótulos de "1" a "6"
        let n1 = graph.add_node("1");
        let n2 = graph.add_node("2");
        let n3 = graph.add_node("3");
        let n4 = graph.add_node("4");
        let n5 = graph.add_node("5");
        let n6 = graph.add_node("6");
        
        // Conecta os nós para formar um grafo conectado
        graph.add_edge(n1, n2, ());
        graph.add_edge(n1, n3, ());
        graph.add_edge(n2, n4, ());
        graph.add_edge(n3, n5, ());
        graph.add_edge(n4, n6, ());
        
        MyGraph { graph }
    }

    pub fn dfs_from_node1(&self) -> Vec<&'static str> {
        let start_node = NodeIndex::new(0); // Primeiro nó adicionado ("1")
        let mut dfs = Dfs::new(&self.graph, start_node);
        let mut visited_nodes = vec![];
        
        while let Some(node) = dfs.next(&self.graph) {
            visited_nodes.push(self.graph[node]);
        }
        
        visited_nodes
    }

    /// Gera representação do grafo em formato DOT
    pub fn to_dot(&self) -> String {
        format!("{:?}", petgraph::dot::Dot::new(&self.graph))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_dfs_visits_all_nodes() {
        let g = MyGraph::new();
        let result = g.dfs_from_node1();

        let mut sorted_result = result.clone();
        sorted_result.sort();

        assert_eq!(
            sorted_result,
            vec!["1", "2", "3", "4", "5", "6"],
            "DFS deve visitar todos os nós"
        );
    }

    #[test]
    fn test_dfs_starts_at_node1() {
        let g = MyGraph::new();
        assert_eq!(
            g.dfs_from_node1().first(),
            Some(&"1"),
            "DFS deve iniciar no nó 1"
        );
    }

    #[test]
    fn generate_dot_file() {
        let g = MyGraph::new();
        let dot_content = g.to_dot();
        
        File::create("grafo.dot")
            .and_then(|mut file| file.write_all(dot_content.as_bytes()))
            .expect("Erro ao gerar arquivo DOT");
    }
}