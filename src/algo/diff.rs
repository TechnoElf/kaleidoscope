use std::fmt::Debug;

// http://www.xmailserver.org/diff2.pdf

#[derive(Debug, PartialEq, Eq)]
pub enum Edit<T>
    where T: Debug + PartialEq + Eq {
    Remove(T),
    Keep(T),
    Insert(T)
}

#[derive(Debug)]
pub struct EditGraph<T> {
    a: Vec<T>,
    b: Vec<T>
}

impl<T> EditGraph<T> {
    // TODO: use borrowed Vecs
    pub fn new(a: Vec<T>, b: Vec<T>) -> Self {
        Self {
            a,
            b
        }
    }
}

impl<T> EditGraph<T> 
    where T: Debug + PartialEq + Eq + Clone {
    fn is_match_point(&self, x: usize, y: usize) -> bool {
        if x >= self.a.len() || y >= self.b.len() { return false; }
        self.a[x] == self.b[y]
    }

    fn dijkstra(&self) -> Vec<(usize, usize)> {
        let mut dist = vec![vec![None; self.a.len() + 1]; self.b.len() + 1];
        let mut parent = vec![vec![None; self.a.len() + 1]; self.b.len() + 1];
        let mut visited = vec![vec![false; self.a.len() + 1]; self.b.len() + 1];

        // TODO: use BinaryHeap
        let mut queue = vec![(0, 0)];
        dist[0][0] = Some(0);

        while let Some((node_x, node_y)) = queue.pop() {
            let next_dist = dist[node_y][node_x].unwrap() + 1;

            // right
            if node_x < self.a.len() {
                if next_dist < dist[node_y][node_x + 1].unwrap_or(usize::MAX) {
                    dist[node_y][node_x + 1] = Some(next_dist);
                    parent[node_y][node_x + 1] = Some((node_x, node_y));

                    if !visited[node_y][node_x + 1] {
                        queue.push((node_x + 1, node_y))
                    }
                }
            }

            // down
            if node_y < self.b.len() {
                if next_dist < dist[node_y + 1][node_x].unwrap_or(usize::MAX) {
                    dist[node_y + 1][node_x] = Some(next_dist);
                    parent[node_y + 1][node_x] = Some((node_x, node_y));

                    if !visited[node_y + 1][node_x] {
                        queue.push((node_x, node_y + 1))
                    }
                }
            }

            // diagonal
            if node_x < self.a.len() && node_y < self.b.len() &&
                self.is_match_point(node_x, node_y) {
                if next_dist < dist[node_y + 1][node_x + 1].unwrap_or(usize::MAX) {
                    dist[node_y + 1][node_x + 1] = Some(next_dist);
                    parent[node_y + 1][node_x + 1] = Some((node_x, node_y));

                    if !visited[node_y + 1][node_x + 1] {
                        queue.push((node_x + 1, node_y + 1))
                    }
                }
            }

            visited[node_y][node_x] = true;

            queue.sort_by(|a, b| dist[b.1][b.0].unwrap().cmp(&dist[a.1][a.0].unwrap()));

            if node_x >= self.a.len() && node_y >= self.b.len() { break; }
        }

        let mut path = Vec::new();
        let (mut node_x, mut node_y) = (self.a.len(), self.b.len());
        path.push((node_x, node_y));
        while let Some((parent_x, parent_y)) = parent[node_y][node_x] {
            path.insert(0, (parent_x, parent_y));
            (node_x, node_y) = (parent_x, parent_y);
        }

        path
    }

    fn myers(&self) -> Vec<(usize, usize)> {
        let n = self.a.len() as isize;
        let m = self.b.len() as isize;
        let max = m + n;

        let mut parent = vec![vec![None; n as usize + 1]; m as usize + 1];

        let mut endpoints = Vec::new();
        endpoints.push((-1, 0));

        'outer: for _d in 0..=max {
            let mut cur_endpoints = Vec::new();
            std::mem::swap(&mut endpoints, &mut cur_endpoints);

            for e in cur_endpoints {
                if e.0 + 1 >= 0 && e.1 >= 0 {
                    let (mut x, mut y) = (e.0 + 1, e.1);
                    if 0 <= x && x <= n && 0 <= y && y <= m && parent[y as usize][x as usize].is_none() { parent[y as usize][x as usize] = Some((e.0, e.1)); }

                    while self.is_match_point(x as usize, y as usize) {
                        (x, y) = (x + 1, y + 1);
                        if 0 <= x && x <= n && 0 <= y && y <= m && parent[y as usize][x as usize].is_none() { parent[y as usize][x as usize] = Some((x - 1, y - 1)); }
                    }

                    if x >= n && y >= m { break 'outer; }
                    if !endpoints.contains(&(x, y)) { endpoints.push((x, y)); }
                }

                if e.0 >= 0 && e.1 + 1 >= 0 {
                    let (mut x, mut y) = (e.0, e.1 + 1);
                    if 0 <= x && x <= n && 0 <= y && y <= m && parent[y as usize][x as usize].is_none() { parent[y as usize][x as usize] = Some((e.0, e.1)); }

                    while self.is_match_point(x as usize, y as usize) {
                        (x, y) = (x + 1, y + 1);
                        if 0 <= x && x <= n && 0 <= y && y <= m && parent[y as usize][x as usize].is_none() { parent[y as usize][x as usize] = Some((x - 1, y - 1)); }
                    }

                    if x >= n && y >= m { break 'outer; }
                    if !endpoints.contains(&(x, y)) { endpoints.push((x, y)); }
                }
            }
        }

        let mut path = Vec::new();
        let (mut x, mut y) = (n, m);
        path.push((n as usize, m as usize));
        while let Some((par_x, par_y)) = parent[y as usize][x as usize] {
            (x, y) = (par_x, par_y);
            if x < 0 || y < 0 { break; }
            path.insert(0, (x as usize, y as usize));
        }

        path
    }

    pub fn edit_script(&self) -> Vec<Edit<T>> {
        let mut script = Vec::new();

        let path = self.dijkstra();
        let mut path = path.iter();

        let (mut prev_x, mut prev_y) = path.next().unwrap();
        for (x, y) in path {
            let (d_x, d_y) = (*x - prev_x, *y - prev_y);

            match (d_x, d_y) {
                (1, 0) => script.push(Edit::Remove(self.a[prev_x].clone())),
                (0, 1) => script.push(Edit::Insert(self.b[prev_y].clone())),
                (1, 1) => script.push(Edit::Keep(self.a[prev_x].clone())),
                _ => unreachable!()
            }

            (prev_x, prev_y) = (*x, *y);
        }

        script
    }

    pub fn edit_script_myers(&self) -> Vec<Edit<T>> {
        let mut script = Vec::new();

        let path = self.myers();
        let mut path = path.iter();

        let (mut prev_x, mut prev_y) = path.next().unwrap();
        for (x, y) in path {
            let (d_x, d_y) = (*x - prev_x, *y - prev_y);

            match (d_x, d_y) {
                (1, 0) => script.push(Edit::Remove(self.a[prev_x].clone())),
                (0, 1) => script.push(Edit::Insert(self.b[prev_y].clone())),
                (1, 1) => script.push(Edit::Keep(self.a[prev_x].clone())),
                _ => unreachable!()
            }

            (prev_x, prev_y) = (*x, *y);
        }

        script
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_text_diff() {
        let a = "abcabba".chars().collect();
        let b = "cbabac".chars().collect();
        let edit_graph = EditGraph::new(a, b);
        let expected_script = vec![
            Edit::Insert('c'), Edit::Remove('a'), Edit::Keep('b'),
            Edit::Remove('c'), Edit::Keep('a'), Edit::Keep('b'),
            Edit::Remove('b'), Edit::Keep('a'), Edit::Insert('c')
        ];
        assert_eq!(edit_graph.edit_script(), expected_script);
    }

    #[test]
    fn short_text_diff_myers() {
        let a = "abcabba".chars().collect();
        let b = "cbabac".chars().collect();
        let edit_graph = EditGraph::new(a, b);
        let expected_script = vec![
            Edit::Insert('c'), Edit::Remove('a'), Edit::Keep('b'),
            Edit::Remove('c'), Edit::Keep('a'), Edit::Keep('b'),
            Edit::Remove('b'), Edit::Keep('a'), Edit::Insert('c')
        ];
        assert_eq!(edit_graph.edit_script_myers(), expected_script);
    }

    #[test]
    fn long_text_diff() {
        let a = "jurghuerhgukrshgeuriguiegerguiwrgui".chars().collect();
        let b = "ruieguirghuieugiteuguitouwrehjrguiwrhguiorewh".chars().collect();
        let _ = EditGraph::new(a, b);
    }
}
