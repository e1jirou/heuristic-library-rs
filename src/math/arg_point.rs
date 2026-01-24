#[derive(Debug, Default, Clone, Copy)]
pub struct ArgPoint {
    x: i64,
    y: i64,
}

impl ArgPoint {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn quadrant(&self) -> i32 {
        match (self.x >= 0, self.y >= 0) {
            (true, true) => 0,
            (false, true) => 1,
            (false, false) => 2,
            (true, false) => 3,
        }
    }

    pub fn cmp(&self, other: &ArgPoint) -> std::cmp::Ordering {
        let q1 = self.quadrant();
        let q2 = other.quadrant();
        if q1 != q2 {
            return q1.cmp(&q2);
        }
        (self.y * other.x).cmp(&(self.x * other.y))
    }
}

impl PartialEq for ArgPoint {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for ArgPoint {}

impl PartialOrd for ArgPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ArgPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}
