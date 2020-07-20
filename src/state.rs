use std::cell::RefCell;
use std::rc::Rc;

use crate::canvas::Canvas;
use crate::geometry::{Segment, Shape, Vec2};
use crate::input::Input;
use crate::navigation::{Navigation, NavigationInternal, NavigationObstacle};

pub enum Placing {
    Start,
    End,
    Obstacle(Shape),
}

pub struct State {
    obstacles: Vec<Shape>,
    start: Option<Vec2>,
    end: Option<Vec2>,
    placing: Option<Placing>,
    navigation: Navigation,
    current_path: Vec<Vec2>,
}

const OBSTACLE_PLACING_FINISH_DIST_SQUARED: f64 = 100.;

fn can_add_vertex_to_obstacle(point: Vec2, shape: &Shape) -> bool {
    let length = shape.vertices.len();
    match length {
        0 => true,
        1 => point != shape.vertices[0],
        _ => {
            let new_segment = Segment::new(*shape.vertices.last().unwrap(), point);
            let mut iter = shape.segments().into_iter();
            let mut cross_check_segment_items = length - 2;
            if shape.vertices[0] == point {
                // Finishing
                let first_segment = iter.next().unwrap();
                if first_segment.overlaps_with_p0_to(new_segment.p0) {
                    return false;
                }
                cross_check_segment_items -= 1;
            }
            for cross_check_segment in iter.by_ref().take(cross_check_segment_items) {
                if new_segment.intersect(&cross_check_segment).is_some() {
                    return false;
                }
            }
            let last_segment = iter.next().unwrap();
            if last_segment.p1 == point {
                return false;
            }
            !last_segment.overlaps_with_p1_to(point)
        }
    }
}

impl State {
    pub fn new() -> Rc<RefCell<State>> {
        Rc::new(RefCell::new(State {
            obstacles: vec![],
            start: None,
            end: None,
            placing: None,
            navigation: Navigation::new(vec![]),
            current_path: vec![],
        }))
    }
    pub fn update(&mut self, input: &Input) {
        if input.is_frame_key_pressed("KeyO") {
            self.set_placing(Placing::Obstacle(Shape::new_empty()));
        } else if input.is_frame_key_pressed("KeyS") {
            self.set_placing(Placing::Start);
        } else if input.is_frame_key_pressed("KeyE") {
            self.set_placing(Placing::End);
        };

        if let Some(Placing::Start) = self.placing {
            self.start = Some(input.mouse_pos());
            self.endpoint_updated();
        } else if let Some(Placing::End) = self.placing {
            self.end = Some(input.mouse_pos());
            self.endpoint_updated();
        }

        if let Some(mouse_click) = input.frame_mouse_clicked() {
            self.click(mouse_click.pair());
        }
    }
    pub fn obstacles_updated(&mut self) {
        self.navigation = Navigation::new(
            self.obstacles
                .iter()
                .map(|obstacle| NavigationObstacle::new(obstacle.vertices.clone()))
                .collect(),
        );
        self.find_path();
    }
    pub fn endpoint_updated(&mut self) {
        self.find_path();
    }
    fn find_path(&mut self) {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            self.current_path = if let Some(path) = self.navigation.find_path(start, end) {
                path
            } else {
                vec![]
            }
        } else {
            self.current_path = vec![];
        }
    }
    pub fn set_placing(&mut self, new_placing: Placing) {
        self.placing.replace(new_placing);
    }
    pub fn click(&mut self, mouse_pos: (i32, i32)) {
        #[allow(clippy::single_match)]
        match &mut self.placing {
            Some(Placing::Obstacle(shape)) => {
                let pos = mouse_pos.into();
                let finishing =
                    !shape.is_empty() && shape.vertices[0].dist_squared(pos) < OBSTACLE_PLACING_FINISH_DIST_SQUARED;
                if finishing {
                    if !can_add_vertex_to_obstacle(shape.vertices[0], shape) {
                        return;
                    }
                    if let Some(Placing::Obstacle(shape)) = self.placing.replace(Placing::Obstacle(Shape::new_empty()))
                    {
                        self.obstacles.push(shape);
                        self.obstacles_updated();
                    } else {
                        unreachable!();
                    }
                } else {
                    if !can_add_vertex_to_obstacle(pos, shape) {
                        return;
                    }
                    shape.vertices.push(pos);
                }
            }
            Some(Placing::Start) => {
                self.placing.take();
                self.start = Some(mouse_pos.into());
                self.endpoint_updated();
            }
            Some(Placing::End) => {
                self.placing.take();
                self.end = Some(mouse_pos.into());
                self.endpoint_updated();
            }
            _ => {}
        }
    }
    fn render_obstacles(&self, canvas: &Canvas) {
        for obstacle in &self.obstacles {
            canvas.begin_path();
            if obstacle.is_empty() {
                continue;
            }
            canvas.move_to(*obstacle.vertices.last().unwrap());
            for vertex in &obstacle.vertices {
                canvas.line_to(*vertex);
            }
            canvas.set_fill_style("#CCC");
            canvas.fill();
            canvas.set_stroke_style("#000");
            canvas.stroke();
        }
    }
    fn render_one_endpoint(&self, canvas: &Canvas, point: Vec2, color: &str) {
        canvas.set_fill_style(color);
        canvas.begin_path();
        canvas.circle(point, 3.);
        canvas.fill();
    }
    fn render_endpoints(&self, canvas: &Canvas) {
        if let Some(start) = self.start {
            self.render_one_endpoint(canvas, start, "#0F0");
        }
        if let Some(end) = self.end {
            self.render_one_endpoint(canvas, end, "#F00");
        }
    }
    fn render_placing_obstacle(&self, canvas: &Canvas, input: &Input) {
        if let Some(Placing::Obstacle(shape)) = &self.placing {
            if !shape.is_empty() {
                canvas.begin_path();
                canvas.move_to(shape.vertices[0]);
                for vertex in &shape.vertices[1..] {
                    canvas.line_to(*vertex);
                }
                let mouse_pos = &input.mouse_pos();
                let goal = if mouse_pos.dist_squared(shape.vertices[0]) < OBSTACLE_PLACING_FINISH_DIST_SQUARED {
                    canvas.set_stroke_style("#0C0");
                    shape.vertices[0]
                } else {
                    canvas.set_stroke_style("#999");
                    input.mouse_pos()
                };
                if !can_add_vertex_to_obstacle(goal, shape) {
                    canvas.set_stroke_style("#F00");
                }
                canvas.line_to(goal);
                canvas.stroke();
            }
        }
    }
    fn render_navigation_graph(&self, canvas: &Canvas) {
        canvas.begin_path();
        for segment in self.navigation.internal_paths() {
            canvas.segment(&segment);
        }
        canvas.set_stroke_style("#f00");
        canvas.stroke();
    }
    fn render_current_path(&self, canvas: &Canvas) {
        if !self.current_path.is_empty() {
            canvas.begin_path();
            canvas.move_to(self.current_path[0]);
            for vertex in &self.current_path[1..] {
                canvas.line_to(*vertex);
            }
            canvas.set_stroke_style("#00F");
            canvas.stroke()
        }
    }
    pub fn render(&self, canvas: &Canvas, input: &Input) {
        canvas.clear();
        self.render_obstacles(canvas);
        self.render_placing_obstacle(canvas, input);
        self.render_endpoints(canvas);
        // self.render_navigation_graph(canvas);
        self.render_current_path(canvas);
        web_sys::window().unwrap().document().unwrap().set_title(&format!(
            "{}, {}",
            input.mouse_pos().x,
            input.mouse_pos().y
        ));
    }
}
