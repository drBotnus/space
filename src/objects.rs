use rand::Rng;

#[derive(Debug)]
pub struct SpaceObject {
    pos: (i32, i32),
}

impl SpaceObject {
    pub fn new(nx: i32, ny: i32) -> Self {
        Self { pos: (nx, ny) }
    }

    pub fn update(&mut self) {
        self.pos.1 += 1
    }
    pub fn get_pos(&self) -> (i32, i32) {
        self.pos
    }
}

#[derive(Debug)]
pub struct ObjectField {
    field_bounds: Rect,
    pub object_set: Vec<SpaceObject>,
}

impl ObjectField {
    pub fn update(&mut self) {
        self.object_set.retain_mut(|obj| {
            if obj.get_pos().1 <= self.field_bounds.bottom() {
                obj.update();
                true
            } else {
                false
            }
        });
        let mut rng = rand::thread_rng();
        let new_obj: SpaceObject = SpaceObject::new(rng.gen_range(0..self.field_bounds.width()), 0);
        self.object_set.push(new_obj);
    }

    pub fn get_data(&mut self) -> &mut Vec<SpaceObject> {
        &mut self.object_set
    }

    pub fn new(bounds: Rect, set: Vec<SpaceObject>) -> Self {
        Self {
            field_bounds: bounds,
            object_set: set,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub offset: (i32, i32),
    pub bounds: (i32, i32),
}

impl Rect {
    pub fn top(&self) -> i32 {
        self.offset.1
    }
    pub fn bottom(&self) -> i32 {
        self.offset.1 + self.bounds.1
    }
    pub fn left(&self) -> i32 {
        self.offset.0
    }
    pub fn right(&self) -> i32 {
        self.offset.0 + self.bounds.0
    }
    pub fn width(&self) -> i32 {
        self.bounds.0
    }
    pub fn height(&self) -> i32 {
        self.bounds.1
    }
    pub fn contains(&self, bounds: (i32, i32)) -> bool {
        (bounds.0 >= self.offset.0 && bounds.0 < self.right())
            && (bounds.1 >= self.offset.1 && bounds.1 < self.bottom())
    }
}
