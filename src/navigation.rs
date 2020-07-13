use crate::geometry::Vec2;

pub struct NavigationObstacle {
    vertices: Vec<Vec2>,
    /// true means cannot pass through; false means cannot leave
    inside: bool,
}

impl NavigationObstacle {
    pub fn new(vertices: Vec<Vec2>) -> Self {
        NavigationObstacle { vertices, inside: true }
    }
}

pub struct ShapeVertex {
    shape_index: usize,
    vertex_index: usize,
}

pub enum EndPoint {
    Free(Vec2),
    ShapeVertex(ShapeVertex),
}

pub struct Navigation {
    obstacles: Vec<NavigationObstacle>,
}

impl Navigation {
    pub fn new(obstacles: Vec<NavigationObstacle>) -> Self {
        Navigation { obstacles }
    }
    pub fn find_path(&self, start: EndPoint, end: EndPoint) -> Vec<Vec2> {
        vec![Vec2::new(0., 0.)]
    }
}
