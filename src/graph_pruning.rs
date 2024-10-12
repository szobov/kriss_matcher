use petgraph::{Graph, Undirected};
use rustworkx_core::connectivity::core_number;

use crate::point_cloud::PointCloud;

pub fn correspondance_graph_pruning(
    correspondance: &Vec<(u64, u64)>,
    source: &PointCloud,
    target: &PointCloud,
    distance_noise_threshold: f64,
) -> Vec<(u64, u64)> {
    // TODO: CSR was proposed, but I'm, unsure it's better here.
    //       petgraph supports it: https://web.archive.org/web/20240824094053/
    //       https://docs.rs/petgraph/latest/petgraph/csr/struct.Csr.html
    let mut correspondance_graph = Graph::<(u64, u64), (), Undirected>::new_undirected();
    let node_indexes: Vec<_> = correspondance
        .iter()
        .map(|&corr| correspondance_graph.add_node(corr))
        .collect();

    for (i, (source_index_i, target_index_i)) in correspondance.iter().enumerate() {
        for (j, (source_index_j, target_index_j)) in correspondance.iter().enumerate().skip(i + 1) {
            let source_point_i = &source.points[*source_index_i as usize];
            let source_point_j = &source.points[*source_index_j as usize];
            let target_point_i = &target.points[*target_index_i as usize];
            let target_point_j = &target.points[*target_index_j as usize];

            let distance_source = source_point_i.distance(source_point_j);
            let distance_target = target_point_i.distance(target_point_j);

            if (distance_target - distance_source).abs() < 2.0 * distance_noise_threshold {
                correspondance_graph.add_edge(node_indexes[i], node_indexes[j], ());
            }
        }
    }

    let cores = core_number(&correspondance_graph);
    let mut filtered_correspondances = Vec::new();
    if let Some(max_core_number) = &cores.values().max() {
        for (node_id, _) in cores.iter().filter(|(_, v)| v == max_core_number) {
            if let Some(&filtered_correspondance) = &correspondance_graph.node_weight(*node_id) {
                filtered_correspondances.push(filtered_correspondance.clone());
            }
        }
    } else {
        panic!("No cores where found.")
    }
    filtered_correspondances
}
