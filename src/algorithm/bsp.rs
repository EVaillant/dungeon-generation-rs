use crate::algorithm::{Error, MapGenerator, Point, Rectangle, TileType};
use rand::prelude::*;
use std::cell::RefCell;

pub struct BspMapGenerator {
    width: u32,
    height: u32,
    nb_iteration: u32,
    min_split_size: u32,
    min_distance_from_vertex: u32,
    data: Vec<Vec<TileType>>,
    rng: RefCell<ThreadRng>,
}

impl BspMapGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        let line = vec![TileType::Wall; height as usize];
        let data = vec![line; width as usize];
        Self {
            width,
            height,
            nb_iteration: 7,
            min_split_size: 5,
            min_distance_from_vertex: 1,
            data,
            rng: RefCell::new(rand::rng()),
        }
    }

    pub fn set_nb_iteration(&mut self, nb: u32) {
        self.nb_iteration = nb;
    }

    pub fn set_min_split_size(&mut self, size: u32) {
        self.min_split_size = size;
    }

    pub fn set_min_distnace_from_vertex(&mut self, value: u32) {
        self.min_distance_from_vertex = value;
    }

    fn split(&self, c0: u32, c1: u32) -> Option<u32> {
        if c1 - c0 + 1 < 2 * self.min_split_size + 1 {
            None
        } else {
            Some(
                self.rng
                    .borrow_mut()
                    .random_range(c0 + self.min_split_size..=c1 - self.min_split_size),
            )
        }
    }

    fn split_rect_width(
        &self,
        unsplitted: &mut Vec<Rectangle>,
        splitted: &mut Vec<Rectangle>,
        rectangle: Rectangle,
    ) {
        if let Some(value) = self.split(rectangle.bottom_left.x, rectangle.top_right.x) {
            let x = if value - rectangle.bottom_left.x + 1 > rectangle.top_right.x - value + 1 {
                (value - 1, value + 1)
            } else {
                (value, value + 2)
            };
            splitted.push(Rectangle::from_corner(
                rectangle.bottom_left,
                Point::new(x.0, rectangle.top_right.y),
            ));

            splitted.push(Rectangle::from_corner(
                Point::new(x.1, rectangle.bottom_left.y),
                rectangle.top_right,
            ));
        } else {
            unsplitted.push(rectangle);
        }
    }

    fn split_rect_height(
        &self,
        unsplitted: &mut Vec<Rectangle>,
        splitted: &mut Vec<Rectangle>,
        rectangle: Rectangle,
    ) {
        if let Some(value) = self.split(rectangle.bottom_left.y, rectangle.top_right.y) {
            let y = if value - rectangle.bottom_left.y + 1 > rectangle.top_right.y - value + 1 {
                (value - 1, value + 1)
            } else {
                (value, value + 2)
            };

            splitted.push(Rectangle::from_corner(
                rectangle.bottom_left,
                Point::new(rectangle.top_right.x, y.0),
            ));
            splitted.push(Rectangle::from_corner(
                Point::new(rectangle.bottom_left.x, y.1),
                rectangle.top_right,
            ));
        } else {
            unsplitted.push(rectangle);
        }
    }

    fn generate_rooms(&self) -> Vec<Rectangle> {
        let mut splitted_rect = vec![Rectangle::from_corner(
            Point::new(1, 1),
            Point::new(self.width - 2, self.height - 2),
        )];
        let mut result = Vec::new();

        for _i in 0..self.nb_iteration {
            let mut new_splitted_rect = Vec::new();
            for rectangle in splitted_rect.into_iter() {
                if rectangle.width() > rectangle.height() {
                    self.split_rect_width(&mut result, &mut new_splitted_rect, rectangle);
                } else {
                    self.split_rect_height(&mut result, &mut new_splitted_rect, rectangle);
                }
            }
            splitted_rect = new_splitted_rect;
        }
        result.append(&mut splitted_rect);
        result
    }

    fn generate_doors(&self, rooms: &[Rectangle]) -> Vec<Point> {
        let mut result = Vec::new();
        for room in rooms.iter() {
            for other_room in rooms.iter().filter(|item| *item != room) {
                let possible_doors = (0..room.width())
                    .map(|i| Point::new(room.bottom_left.x + i, room.top_right.y + 1))
                    .filter(|door| self.check_door(other_room, door) && self.check_door(room, door))
                    .collect::<Vec<_>>();
                if !possible_doors.is_empty() {
                    let doors_idx = self.rng.borrow_mut().random_range(0..=possible_doors.len());
                    if doors_idx > 0 {
                        result.push(possible_doors[doors_idx - 1]);
                    }
                }

                let possible_doors = (0..room.width())
                    .map(|i| Point::new(room.top_right.x + 1, room.bottom_left.y + i))
                    .filter(|door| self.check_door(other_room, door) && self.check_door(room, door))
                    .collect::<Vec<_>>();
                if !possible_doors.is_empty() {
                    let doors_idx = self.rng.borrow_mut().random_range(0..=possible_doors.len());
                    if doors_idx > 0 {
                        result.push(possible_doors[doors_idx - 1]);
                    }
                }
            }
        }
        result
    }

    fn draw_room_floor(&mut self, rooms: &[Rectangle]) {
        for room in rooms.iter() {
            for x in room.bottom_left.x..=room.top_right.x {
                for y in room.bottom_left.y..=room.top_right.y {
                    self.set_title(Point::new(x, y), TileType::Floor);
                }
            }
        }
    }

    fn draw_door(&mut self, doors: &[Point]) {
        for door in doors.iter() {
            self.set_title(*door, TileType::Door);
        }
    }

    fn set_title(&mut self, point: Point, value: TileType) {
        self.data[point.x as usize][point.y as usize] = value;
    }

    fn check_door(&self, room: &Rectangle, door: &Point) -> bool {
        if room.bottom_left.y == door.y + 1 || room.top_left.y + 1 == door.y {
            room.bottom_left.x <= door.x
                && room.bottom_right.x >= door.x
                && room.bottom_right.x - door.x >= self.min_distance_from_vertex
                && door.x - room.bottom_left.x >= self.min_distance_from_vertex
        } else if room.bottom_left.x == door.x + 1 || room.bottom_right.x + 1 == door.x {
            room.bottom_left.y <= door.y
                && room.top_left.y >= door.y
                && room.top_left.y - door.y >= self.min_distance_from_vertex
                && door.y - room.bottom_left.y >= self.min_distance_from_vertex
        } else {
            false
        }
    }
}

impl MapGenerator for BspMapGenerator {
    fn generate(&mut self) -> Result<(), Error> {
        let rooms = self.generate_rooms();
        let doors = self.generate_doors(&rooms);

        // remove room without door
        let rooms = rooms
            .into_iter()
            .filter(|room| doors.iter().any(|door| self.check_door(room, door)))
            .collect::<Vec<_>>();

        self.draw_door(&doors);
        self.draw_room_floor(&rooms);

        Ok(())
    }

    fn get_title(&self, x: u32, y: u32) -> TileType {
        self.data[x as usize][y as usize]
    }
}
