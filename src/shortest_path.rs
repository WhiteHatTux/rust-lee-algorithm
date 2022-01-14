mod solution {
    use std::rc::Rc;

    #[derive(Debug)]
    #[derive(PartialEq)]
    struct Node {
        position: (u32, u32),
        parent: Option<Rc<Node>>,
        path_length: u32,
    }

    impl Node {
        pub(crate) fn get_result_path(&self, mut results: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
            results.push(self.position);
            return if self.parent.is_some() {
                self.parent.as_ref().unwrap().get_result_path(results)
            } else {
                results
            };
        }
    }


    pub fn find_path(maze: Vec<Vec<u32>>, start: (u32, u32), dest: (u32, u32)) -> (u32, Vec<(u32, u32)>) {
        check_for_plausibility(&maze, start, dest);
        if maze.len() == 0 {
            return (0, vec![]);
        }
        let mut nodes = vec![Rc::new(Node {
            position: start,
            parent: Option::None,
            path_length: 0,
        })];
        let mut walked: Vec<(u32, u32)> = vec![];
        let mut found = false;
        let mut path_length = 1;
        let mut winning_node: Option<Rc<Node>> = Option::None;
        while !found {
            let mut temp_nodes = vec![];
            {
                nodes.iter()
                    .filter(|node| node.path_length == path_length - 1)
                    .for_each(|node| {
                        if node.position.eq(&dest) {
                            found = true;
                            winning_node = Option::Some(Rc::clone(&node));
                        }

                        add_new_node(&mut temp_nodes, &mut walked, &node, get_upper(&maze, node.position));
                        add_new_node(&mut temp_nodes, &mut walked, &node, get_right(&maze, node.position));
                        add_new_node(&mut temp_nodes, &mut walked, &node, get_down(&maze, node.position));
                        add_new_node(&mut temp_nodes, &mut walked, &node, get_left(&maze, node.position));
                    });
            }
            if temp_nodes.len() == 0 {
                println!("No path exists from start to dest");
                break;
            }
            temp_nodes.into_iter().for_each(|temp_node| nodes.push(temp_node));
            path_length += 1;
            println!("nodes count {}", nodes.len());
            //nodes.iter().for_each(|node| print!("pos:{:?},par:{:?}|", node.position, node.parent.as_ref().map(|par| par.position).unwrap_or()));
            println!();

            println!("last nodes position {:?}", nodes.last().unwrap().position);
            if nodes.len() > (maze.len() * maze.get(0).unwrap().len()) {
                panic!("Algorithm is adding to many nodes");
            }
        }

        match winning_node {
            Some(winning_node_value) => {
                let mut res = winning_node_value.get_result_path(vec![] as Vec<(u32, u32)>);
                res.reverse();
                (winning_node_value.path_length, res)
            }
            None => (0, vec![])
        }
    }

    fn check_for_plausibility(maze: &Vec<Vec<u32>>, start: (u32, u32), dest: (u32, u32)) {
        if start.0 as usize > maze.get(0).unwrap().len() {
            panic!("start x is outside the maze");
        }
        if start.1 as usize > maze.len() {
            panic!("start y is outside the maze");
        }
        if dest.0 as usize > maze.get(0).unwrap().len() {
            panic!("dest x is outside the maze");
        }
        if dest.1 as usize > maze.len() {
            panic!("dest y is outside the maze");
        }
    }


    fn add_new_node(temp_nodes: &mut Vec<Rc<Node>>, walked: &mut Vec<(u32, u32)>, node: &Rc<Node>, optional_step: Option<(u32, u32)>) {
        match optional_step {
            Some(new_position) => {
                let already_exists = walked.iter()
                    .find(|walked_position| walked_position.0.eq(&new_position.0) && walked_position.1.eq(&new_position.1)).is_none();

                if already_exists {
                    walked.push(new_position);
                    temp_nodes.push(Rc::new(Node {
                        position: new_position,
                        path_length: node.path_length + 1,
                        parent: Option::Some(Rc::clone(node)),
                    }));
                };
            }
            None => {}
        }
    }


    fn get_upper(maze: &Vec<Vec<u32>>, position: (u32, u32)) -> Option<(u32, u32)> {
        let (x, y) = position;
        check_if_node_walkable(maze, y as i32 - 1, x as i32)
    }

    fn get_right(maze: &Vec<Vec<u32>>, position: (u32, u32)) -> Option<(u32, u32)> {
        let (x, y) = position;
        check_if_node_walkable(maze, y as i32, x as i32 + 1)
    }

    fn get_down(maze: &Vec<Vec<u32>>, position: (u32, u32)) -> Option<(u32, u32)> {
        let (x, y) = position;
        check_if_node_walkable(maze, y as i32 + 1, x as i32)
    }

    fn get_left(maze: &Vec<Vec<u32>>, position: (u32, u32)) -> Option<(u32, u32)> {
        let (x, y) = position;
        check_if_node_walkable(maze, y as i32, x as i32 - 1)
    }

    fn check_if_node_walkable(maze: &Vec<Vec<u32>>, new_y: i32, new_x: i32) -> Option<(u32, u32)> {
        if new_y.is_negative() || new_x.is_negative() {
            return Option::None;
        }
        let node_exists = match maze.get(new_y as usize) {
            Some(line) => match line.get(new_x as usize) {
                Some(field) => field.eq(&(0 as u32)),
                None => false
            },
            None => false
        };
        if node_exists {
            Option::Some((new_x as u32, new_y as u32))
        } else {
            Option::None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_maze() {
        let vec = vec![
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0]];
        assert_eq!(solution::find_path(vec, (2, 2), (7, 4)), (9, vec![(2, 2), (3, 2), (3, 3), (3, 4), (4, 4), (5, 4), (5, 5), (6, 5), (7, 5), (7, 4)]));
    }

    #[test]
    fn simple_maze_no_path() {
        let vec = vec![
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 1, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0]];
        assert_eq!(solution::find_path(vec, (2, 2), (7, 4)), (0, vec![]));
    }

    #[test]
    fn big_manual_maze() {
        let big_vec = vec![
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 0, 0, 0, 1, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 1, 1, 1, 0],
            vec![0, 0, 1, 0, 0, 0, 1, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 0, 0, 0]];
        solution::find_path(big_vec, (2, 2), (7, 34));
    }
    /*
        #[test]
        fn huge_random_maze() {
            let max_x =
        }*/
}

