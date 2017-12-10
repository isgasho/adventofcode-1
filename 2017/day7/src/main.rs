extern crate regex;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn get_file() -> Result<String, &'static str> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Error reading file");
    }
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    Ok(contents)
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<&Node>, // actually connects the references
    children_data: Vec<String>, // used in the intial pass
}

impl Node {
    fn add_child(&mut self, node: &Node) {
        self.children.push(node);
    }
}

fn main() {
    match get_file() {
        Ok(contents) => {
            let lines = contents
                .split("\n")
                .filter(|&s| s.len() != 0)
                .collect::<Vec<&str>>();

            let mut nodes: Vec<Node> = Vec::new();

            let re = Regex::new(r"(\w+) \((\d+)\)( -> (.*))?").unwrap(); // uses 'raw string'

            for datum in lines {

                // let datum = lines.get(0).unwrap();

                // groups;
                // test (00) -> test, test
                // 1     2      4
                let capt = re.captures(datum).unwrap();

                let mut node = Node {
                    name: String::from(capt.get(1).unwrap().as_str()),
                    weight: capt.get(2)
                        .unwrap()
                        .as_str()
                        .parse::<u32>()
                        .unwrap(),
                    children: Vec::new(),
                    children_data: match capt.get(4) {
                        Some(v) => v.as_str()
                            .split(", ")
                            .map(|s| s.to_string())
                            .collect(),
                        None => Vec::new(),
                    },
                };

                nodes.push(node);
                // println!("{}", nodes.len());
                //http://sprunge.us/DQJa?diff
            }

            for node in &nodes {
                if node.children_data.len() != 0 {
                    // convert children_Data into references`
                    for child in &node.children_data {
                        let reference = nodes.iter()
                            .find(|&x| x.name == *child)
                            .unwrap();

                        node.add_child(reference);
                        // println!("{:?}", *reference);
                    }
                }
            }

            // println!("{:#?}", nodes);
        },
        Err(e) => println!("{}", e),
    }
}


// --- Day 7: Recursive Circus ---

// Wandering further through the circuits of the computer, you come upon a tower of programs that have gotten themselves into a bit of trouble. A recursive algorithm has gotten out of hand, and now they're balanced precariously in a large tower.

// One program at the bottom supports the entire tower. It's holding a large disc, and on the disc are balanced several more sub-towers. At the bottom of these sub-towers, standing on the bottom disc, are other programs, each holding their own disc, and so on. At the very tops of these sub-sub-sub-...-towers, many programs stand simply keeping the disc below them balanced but with no disc of their own.

// You offer to help, but first you need to understand the structure of these towers. You ask each program to yell out their name, their weight, and (if they're holding a disc) the names of the programs immediately above them balancing on that disc. You write this information down (your puzzle input). Unfortunately, in their panic, they don't do this in an orderly fashion; by the time you're done, you're not sure which program gave which information.

// For example, if your list is the following:

// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)

// ...then you would be able to recreate the structure of the towers that looks like this:

//                 gyxo
//               /
//          ugml - ebii
//        /      \
//       |         jptl
//       |
//       |         pbga
//      /        /
// tknk --- padx - havc
//      \        \
//       |         qoyq
//       |
//       |         ktlj
//        \      /
//          fwft - cntj
//               \
//                 xhth

// In this example, tknk is at the bottom of the tower (the bottom program), and is holding up ugml, padx, and fwft. Those programs are, in turn, holding up other programs; in this example, none of those programs are holding up any other programs, and are all the tops of their own towers. (The actual tower balancing in front of you is much larger.)

// Before you're ready to help them, you need to make sure your information is correct. What is the name of the bottom program?

// To begin, get your puzzle input.
