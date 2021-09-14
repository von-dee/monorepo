// TODO
// use crate::algorithms::filter_results::filter_results;
// use serde::Serialize;
use std::{collections::HashMap, vec};

#[test]
fn it_works_in_the_typical_case() {
    let _result: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    let mut filter: HashMap<String, Vec<u8>> = HashMap::new();
    let _ = filter.insert("Polywrap".to_string(), vec![1, 2, 3]);
    let _ = filter.insert("Web3API".to_string(), vec![4, 5, 6]);
    let _ = filter.insert("SDK".to_string(), vec![7, 8, 9]);
}

// #[derive(Debug, Default, Clone, PartialEq, Serialize)]
// pub struct RootA {
//     pub prop1: String,
//     pub prop2: String,
// }

// #[derive(Debug, Default, Clone, PartialEq, Serialize)]
// pub struct RootB {
//     pub prop3: i32,
//     pub prop4: Deep,
// }

// #[derive(Debug, Default, Clone, PartialEq, Serialize)]
// pub struct Deep {
//     pub deep: f32,
// }

// #[derive(Debug, Default, Clone, PartialEq, Serialize)]
// pub struct Res {
//     pub root_a: RootA,
//     pub root_b: RootB,
// }

// impl std::fmt::Display for Res {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> core::fmt::Result {
//         let mut formatted = String::new();
//         formatted.push_str(&format!(
//             "{{\
// 			root_a: {:#?}, \
// 			root_b: {:#?}, \
// 			}}",
//             self.root_a.clone(),
//             self.root_b.clone(),
//         ));
//         write!(f, "{}", formatted)
//     }
// }

// #[test]
// fn it_works_in_the_typical_case() {
//     let deep = Deep { deep: 1.5 };
//     let root_a = RootA {
//         prop1: "hey".to_string(),
//         prop2: "heyu".to_string(),
//     };
//     let root_b = RootB {
//         prop3: 5,
//         prop4: deep,
//     };
//     let result: Option<Res> = Some(Res { root_a, root_b });
//     let mut filter: HashMap<String, Option<Res>> = HashMap::new();

//     let filtered_result = filter_results(result, filter).unwrap();
// }
