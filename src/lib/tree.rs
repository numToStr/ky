// // Thanks: https://vallentin.dev/2019/05/14/pretty-print-tree
// use std::fmt;
//
// pub struct Tree {
//     branches: Vec<String>,
// }
//
// impl Tree {
//     pub fn new(b: Vec<String>) -> Self {
//         Self { branches: b }
//     }
// }
//
// struct Node {
//     name: &'static str,
//     children: Vec<Node>,
// }
//
// impl Node {
//     fn new(name: &'static str, children: Vec<Node>) -> Node {
//         Node { name, children }
//     }
// }
//
// impl fmt::Display for Node {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.name)
//     }
// }
//
// fn pprint_tree(node: &Node) {
//     fn pprint_tree(node: &Node, prefix: String, last: bool) {
//         let prefix_current = if last { "`- " } else { "|- " };
//
//         println!("{}{}{}", prefix, prefix_current, node);
//
//         let prefix_child = if last { "   " } else { "|  " };
//         let prefix = prefix + prefix_child;
//
//         if !node.children.is_empty() {
//             let last_child = node.children.len() - 1;
//
//             for (i, child) in node.children.iter().enumerate() {
//                 pprint_tree(&child, prefix.to_string(), i == last_child);
//             }
//         }
//     }
//
//     pprint_tree(node, "".to_string(), true);
// }
//
// // fn example() {
// //     let tree = Node::new(
// //         "Root",
// //         vec![
// //             Node::new(
// //                 "Node 1",
// //                 vec![
// //                     Node::new(
// //                         "Node 1.1",
// //                         vec![Node::new(
// //                             "Node 1.1.1",
// //                             vec![
// //                                 Node::new("Node 1.1.1.1", vec![]),
// //                                 Node::new("Node 1.1.1.2", vec![]),
// //                             ],
// //                         )],
// //                     ),
// //                     Node::new("Node 1.2", vec![]),
// //                     Node::new("Node 1.3", vec![Node::new("Node 1.3.1", vec![])]),
// //                     Node::new(
// //                         "Node 1.4",
// //                         vec![
// //                             Node::new("Node 1.4.1", vec![]),
// //                             Node::new(
// //                                 "Node 1.4.2",
// //                                 vec![
// //                                     Node::new("Node 1.4.2.1", vec![]),
// //                                     Node::new(
// //                                         "Node 1.4.2.2",
// //                                         vec![Node::new("Node 1.4.2.2.1", vec![])],
// //                                     ),
// //                                 ],
// //                             ),
// //                         ],
// //                     ),
// //                 ],
// //             ),
// //             Node::new(
// //                 "Node 2",
// //                 vec![Node::new("Node 2.1", vec![]), Node::new("Node 2.2", vec![])],
// //             ),
// //             Node::new("Node 3", vec![]),
// //         ],
// //     );
// //
// //     pprint_tree(&tree);
// // }
