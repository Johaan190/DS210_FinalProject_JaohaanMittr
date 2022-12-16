     use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::collections::HashMap;
    
    fn main(){
    
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    
    {
        let mut graph: HashMap<i32,Vec<i32>> = HashMap::new();
        println!("Read the graph network data");
        
        if let Ok(lines) = read_lines("roadNet-CA.txt") {
            for line in lines {
                if let Ok(ip) = line {
            if ip.chars().nth(0).unwrap() == '#' {
                continue;		
            }
            let vec: Vec<&str> = ip.split_whitespace().collect();
            let from: i32 = vec[0].to_string().parse().unwrap();
            let to: i32 = vec[1].to_string().parse().unwrap();
            if !graph.contains_key(&from) {
                let mut subvec = Vec::new();
                subvec.push(to);
                graph.insert(from,subvec,);
            } else {
                let mut subvec = graph[&from].clone();
                subvec.push(to);
                graph.insert(from,subvec,);
            }
                }
            }
        }
        let start: i32 = 4;
        let mut goal: i32 = 30;
        println!("Reading finish. Now, searching the path from {} to {} using Breadth First Search",start,goal);
        let mut queue: Vec<i32> = vec![];
        queue.push(start);
        let mut parent: HashMap<i32,i32> = HashMap::new();
        let mut explored: Vec<i32> = vec![];
        explored.push(start);
        while queue.len() > 0 {
            let vertex: i32 = queue[0];
            queue.remove(0);
        if vertex == goal {
            break;	
        }
            for neighbor in &graph[&vertex] {
            if !explored.contains(&neighbor) {
                explored.push(*neighbor);
                parent.insert(*neighbor, vertex);
                queue.push(*neighbor);
            }
        }
        }
        println!("Breadth First Search finished!");
        let mut path: Vec<i32> = vec![];
        path.push(goal);
        while parent.contains_key(&goal) {
            goal = parent[&goal];
            path.push(goal);
        }
        path.reverse();
        println!("Constructed Path:");
        for p in path {
            print!("{} ",p);
        }
        println!(" ");
    }
    }   