use std::f64::consts::PI;

use bv::BitVec;
use noisy_float::prelude::*;
use noisy_float::types::N64;

use crate::a_star::{a_star, AStarInput};
use crate::geometry::{Angle, Segment, Shape, ShapeWindingOrder, Vec2};

pub struct NavigationObstacle {
    shape: Shape,
    concave_vertices: BitVec,
}

impl NavigationObstacle {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        let mut shape = Shape::new(vertices);
        if let ShapeWindingOrder::Clockwise = shape.winding_order() {
            shape.reverse();
        }
        let mut concave_vertices = BitVec::with_capacity(shape.vertices.len() as u64);
        for (vertex_index, vertex) in shape.vertices.iter().enumerate() {
            let prev_direction = (shape.prev_vertex(vertex_index) - *vertex).direction();
            let next_direction = (shape.next_vertex(vertex_index) - *vertex).direction();
            let theta = next_direction - prev_direction; // Inner angle
            concave_vertices.push(theta.as_radians() < PI);
        }
        NavigationObstacle {
            // shape: Shape::new(expanded_vertices),
            shape,
            concave_vertices,
        }
    }
    pub fn expand(&self, delta: f64, resolution: f64) -> Self {
        debug_assert!(delta > 0.);
        debug_assert!(resolution > 0.);
        let mut concave_vertices = BitVec::new();
        let mut expanded_vertices = vec![];
        for (vertex_index, vertex) in self.shape.vertices.iter().enumerate() {
            let prev_direction = (self.shape.prev_vertex(vertex_index) - *vertex).direction();
            let next_direction = (self.shape.next_vertex(vertex_index) - *vertex).direction();
            let theta = next_direction - prev_direction; // Inner angle
            let is_concave = theta.as_radians() < PI;
            if is_concave {
                let start_direction = prev_direction - Angle::from_radians_bounded(PI / 2.);
                let end_direction = next_direction + Angle::from_radians_bounded(PI / 2.);
                let angle_diff = (start_direction - end_direction).as_radians();
                if angle_diff != 2. * PI {
                    let steps = (angle_diff / resolution).round().max(1.);
                    let step_angle = Angle::from_radians_bounded(angle_diff / steps);
                    let mut current_direction = start_direction;
                    for _ in 0..(steps as usize) + 1 {
                        expanded_vertices.push(*vertex + Vec2::dir_mag(current_direction, delta));
                        concave_vertices.push(true);
                        current_direction = current_direction - step_angle;
                    }
                }
            } else {
                let theta_prime = theta.explementary(); // Outer angle
                let side_length = delta / theta_prime.as_radians().sin();
                expanded_vertices.push(
                    *vertex + Vec2::dir_mag(prev_direction, side_length) + Vec2::dir_mag(next_direction, side_length),
                );
                concave_vertices.push(false);
            }
        }
        NavigationObstacle {
            shape: Shape::new(expanded_vertices),
            concave_vertices,
        }
    }
}

struct Node {
    /// # Important
    /// Since connections stored here are shared between multiple path finding sessions, there need
    /// to be a way for us to dynamically inject starting point and ending point into this Vec.
    /// Luckily, we don't necessarily need to have starting point injected, as the path will only
    /// ever leave the starting point, so other nodes do not need to provide a connection towards
    /// it. It is, however, necessarily that we inject the ending point. In order to prevent copy
    /// during the actual path finding, we will prematurely add the node_id of the ending point into
    /// this connection, and control whether the connection exists by slicing out the ending point
    /// during the actual path finding. It is possible to do so, because the node_id of the ending
    /// point is always len() + 1.
    connections: Vec<usize>,
    position: Vec2,
}

type NavigationGraph = Vec<Node>;

#[derive(Debug, Clone, Copy)]
pub struct ShapeVertexIndex {
    shape_index: usize,
    vertex_index: usize,
}

pub struct Navigation {
    obstacles: Vec<NavigationObstacle>,
    navigation_graph: NavigationGraph,
}

fn bound_angle(angle: f64) -> f64 {
    if angle > PI {
        angle - 2. * PI
    } else if angle < -PI {
        angle + 2. * PI
    } else {
        angle
    }
}

fn is_in_connectable_range(obstacle: &NavigationObstacle, vertex: Vec2, vertex_index: usize, target: Vec2) -> bool {
    let a = (obstacle.shape.prev_vertex(vertex_index) - vertex).atan2();
    let b = (obstacle.shape.next_vertex(vertex_index) - vertex).atan2();
    let c = (target - vertex).atan2();
    let p = a <= b;
    let q = b <= c;
    let r = a < c;
    let ap = bound_angle(a + PI);
    let bp = bound_angle(b + PI);
    let s = ap <= bp;
    let t = bp <= c;
    let u = ap < c;
    p ^ q ^ r && s ^ t ^ u
}

impl Navigation {
    fn build_navigation_graph(obstacles: &[NavigationObstacle]) -> NavigationGraph {
        // To build the navigation graph, we cast a line from each vertex0 to every other vertex1,
        // and find intersections with each intersecting_segment.
        let nodes_count = obstacles
            .iter()
            .fold(0, |count, obstacle| count + obstacle.shape.vertices.len());
        let mut navigation_graph: NavigationGraph = Vec::with_capacity(nodes_count);
        for (obstacle0_index, obstacle0) in obstacles.iter().enumerate() {
            for (vertex0_index, vertex0) in obstacle0.shape.vertices.iter().enumerate() {
                // Every vertex0
                let mut node = Node {
                    connections: vec![],
                    position: *vertex0,
                };
                if !obstacle0.concave_vertices[vertex0_index as u64] {
                    navigation_graph.push(node);
                } else {
                    // Only continue if concave
                    // See docs for field `connections`
                    node.connections.push(nodes_count + 1);
                    let mut node1_id = 0;
                    for (obstacle1_index, obstacle1) in obstacles[..(obstacle0_index + 1)].iter().enumerate() {
                        let vertex1_slice = if obstacle0_index == obstacle1_index {
                            &obstacle1.shape.vertices[..vertex0_index]
                        } else {
                            &obstacle1.shape.vertices
                        };
                        'next_vertex: for (vertex1_index, vertex1) in vertex1_slice.iter().enumerate() {
                            // To any other vertex1
                            node1_id += 1;

                            if !obstacle1.concave_vertices[vertex1_index as u64] {
                                // If convex, just skip over
                                continue;
                            }

                            if !is_in_connectable_range(obstacle0, *vertex0, vertex0_index, *vertex1) {
                                continue;
                            }
                            if !is_in_connectable_range(obstacle1, *vertex1, vertex1_index, *vertex0) {
                                continue;
                            }

                            let segment = Segment::new(*vertex0, *vertex1);

                            for (intersecting_obstacle_index, intersecting_obstacle) in obstacles.iter().enumerate() {
                                for (intersecting_segment_index, intersecting_segment) in
                                    intersecting_obstacle.shape.segments().into_iter().enumerate()
                                {
                                    if obstacle0_index == obstacle1_index
                                        && obstacle1_index == intersecting_obstacle_index
                                    {
                                        // All same obstacle
                                        if vertex0_index - 1 == vertex1_index
                                            && intersecting_segment_index == vertex1_index
                                        {
                                            continue;
                                        }
                                        if vertex0_index == obstacle0.shape.vertices.len() - 1
                                            && vertex1_index == 0
                                            && intersecting_segment_index == obstacle0.shape.vertices.len() - 1
                                        {
                                            continue;
                                        }
                                    }
                                    if segment.connective_intersect(&intersecting_segment) {
                                        continue 'next_vertex;
                                    }
                                }
                            }
                            node.connections.push(node1_id - 1);
                            let node0_id = navigation_graph.len();
                            navigation_graph[node1_id - 1].connections.push(node0_id);
                        }
                    }
                    navigation_graph.push(node);
                }
            }
        }
        navigation_graph
    }
    pub fn new(obstacles: Vec<NavigationObstacle>) -> Self {
        let navigation_graph = Navigation::build_navigation_graph(&obstacles);
        Navigation {
            obstacles,
            navigation_graph,
        }
    }
}

struct NavigationAStarInput<'a> {
    navigation_graph: &'a NavigationGraph,
    start_position: Vec2,
    start_connections: Vec<usize>,
    end_position: Vec2,
    end_candidates: BitVec,
}

impl NavigationAStarInput<'_> {
    fn get_node_position(&self, node_id: usize) -> Vec2 {
        if node_id == self.navigation_graph.len() {
            self.start_position
        } else if node_id == self.navigation_graph.len() + 1 {
            self.end_position
        } else {
            self.navigation_graph[node_id].position
        }
    }
}

impl AStarInput for NavigationAStarInput<'_> {
    fn neighbors(&self, node: usize) -> &[usize] {
        if node == self.navigation_graph.len() {
            &self.start_connections
        } else {
            let connections = &self.navigation_graph[node].connections;
            if self.end_candidates[node as u64] {
                connections
            } else {
                // If it does not connect to the ending point, exclude the ending point
                &connections[1..]
            }
        }
    }

    fn distance(&self, from: usize, to: usize) -> N64 {
        n64(self.get_node_position(from).dist(self.get_node_position(to)))
    }

    fn heuristic(&self, node: usize) -> N64 {
        n64(self.get_node_position(node).dist(self.end_position))
    }

    fn len(&self) -> usize {
        self.navigation_graph.len() + 2
    }

    fn start(&self) -> usize {
        self.navigation_graph.len()
    }

    fn end(&self) -> usize {
        self.navigation_graph.len() + 1
    }
}

impl Navigation {
    fn intersects_with_obstacle(&self, segment: Segment) -> bool {
        for intersecting_obstacle in &self.obstacles {
            for intersecting_segment in intersecting_obstacle.shape.segments() {
                if segment.connective_intersect(&intersecting_segment) {
                    return true;
                }
            }
        }
        false
    }
    pub fn find_path(&self, start: Vec2, end: Vec2) -> Option<Vec<Vec2>> {
        if !self.intersects_with_obstacle(Segment::new(start, end)) {
            return Some(vec![start, end]);
        }
        let mut node_id = 0;
        let mut start_connections = vec![];
        let mut end_candidates = BitVec::new_fill(false, self.navigation_graph.len() as u64);
        for connecting_obstacle in &self.obstacles {
            for (connecting_vertex_index, connecting_vertex) in connecting_obstacle.shape.vertices.iter().enumerate() {
                node_id += 1;
                if !connecting_obstacle.concave_vertices[connecting_vertex_index as u64] {
                    // Skip convex
                    continue;
                }
                if is_in_connectable_range(connecting_obstacle, *connecting_vertex, connecting_vertex_index, start)
                    && !self.intersects_with_obstacle(Segment::new(start, *connecting_vertex))
                {
                    start_connections.push(node_id - 1);
                }
                if is_in_connectable_range(connecting_obstacle, *connecting_vertex, connecting_vertex_index, end)
                    && !self.intersects_with_obstacle(Segment::new(end, *connecting_vertex))
                {
                    end_candidates.set((node_id - 1) as u64, true);
                }
            }
        }
        let a_star_input = NavigationAStarInput {
            navigation_graph: &self.navigation_graph,
            start_position: start,
            start_connections,
            end_position: end,
            end_candidates,
        };
        a_star(&a_star_input).map(|vec| {
            vec.into_iter()
                .map(|node_id| a_star_input.get_node_position(node_id))
                .collect()
        })
    }
}

/// Only use this trait if you want to access the internals of a Navigation struct
pub trait NavigationInternal {
    fn internal_navigation_graph(&self) -> Vec<Segment>;
    fn internal_obstacles(&self) -> Vec<&Shape>;
}

impl NavigationInternal for Navigation {
    fn internal_navigation_graph(&self) -> Vec<Segment> {
        self.navigation_graph
            .iter()
            .flat_map(|node0| {
                node0
                    .connections
                    .iter()
                    .skip(1)
                    .map(move |&node_id| Segment::new(node0.position, self.navigation_graph[node_id].position))
            })
            .collect()
    }

    fn internal_obstacles(&self) -> Vec<&Shape> {
        self.obstacles.iter().map(|obstacle| &obstacle.shape).collect()
    }
}
