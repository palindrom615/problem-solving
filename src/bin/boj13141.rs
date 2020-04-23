use problem_solving::utils::*;

fn bfs(graph: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // let mut queue = Vec::new();
    let nodes = graph.len() - 1;
    let mut res = vec![vec![std::usize::MAX; nodes + 1]; nodes + 1];
    for i in 1..graph.len() - 1 {
        for j in i..graph.len() - 1 {

            if graph[i][j] != 0 {
                
            }
        }
    }
    res
}

fn solve(graph: Vec<Vec<usize>>) -> f32 {
    0.
}
fn main() {
    let line: Vec<usize> = read_line();
    let n = line[0];
    let m = line[1];
    let mut graph = vec![vec![0; n + 1]; n + 1];
    for _ in 0..m {
        let edge: Vec<usize> = read_line();
        if graph[edge[0]][edge[1]] < edge[2] {
            graph[edge[0]][edge[1]] = edge[2];
            graph[edge[1]][edge[0]] = edge[2];
        }
    }
    println!("{:?}", graph);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let graph = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 6, 0, 0, 6],
            vec![0, 6, 0, 0, 4, 0],
            vec![0, 0, 0, 4, 6, 0],
            vec![0, 0, 4, 6, 0, 0],
            vec![0, 0, 0, 0, 0, 0],
        ];
        assert_eq!(solve(graph), 9.0);
        let graph = vec![
            vec![0, 0, 0, 0, 0, 0],
            vec![0, 0, 1, 10, 7, 9],
            vec![0, 1, 0, 1, 10, 7],
            vec![0, 10, 1, 0, 1, 10],
            vec![0, 7, 10, 1, 0, 1],
            vec![0, 9, 7, 10, 1, 0],
        ];
        assert_eq!(solve(graph), 6.5);
    }
}
