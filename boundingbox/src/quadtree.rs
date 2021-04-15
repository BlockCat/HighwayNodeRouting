use std::collections::VecDeque;

use crate::{BasicAABB, Coord, AABB};

#[derive(Debug)]
pub struct QuadTree<I: AABB> {
    bounding_boxes: Vec<I>,
    structure: QuadBox,
}

impl<I: AABB + Send + Sync> QuadTree<I> {
    pub fn new(boxes: Vec<I>) -> Self {
        let accumulated: BasicAABB = boxes.iter().map(|x| x.basic()).sum();
        let mut tree = Self {
            structure: QuadBox {
                items: (0..boxes.len()).collect(),
                aabb: accumulated,
                left: None,
                right: None,
                intersection: Vec::new(),
                horizontal: true,
            },
            bounding_boxes: boxes.into_iter().map(Into::into).collect(),
        };

        tree.calculate();

        tree
    }

    fn calculate(&mut self) {
        let mut quads = VecDeque::new();
        quads.push_back(&mut self.structure);

        while let Some(m) = quads.pop_front() {
            // println!("{}", quads.len());
            if let Some((left, right, intersection)) = m.split(&self.bounding_boxes) {

                println!(
                    "left: {}, right: {}, intersect: {}",
                    left.items.len(),
                    right.items.len(),
                    intersection.len()
                );

                m.left = Some(Box::new(left));
                m.right = Some(Box::new(right));
                m.intersection = intersection;
                
                quads.push_back(m.left.as_mut().unwrap().as_mut());
                quads.push_back(m.right.as_mut().unwrap().as_mut());
            }
        }
    }

    pub fn query<R: Into<BasicAABB>>(&self, query: R) -> Vec<I> {
        let query: BasicAABB = query.into();
        let a = query.min();
        let b = query.max();

        vec![]
    }
}

#[derive(Debug)]
struct QuadBox {
    aabb: BasicAABB,
    items: Vec<usize>,
    intersection: Vec<usize>,
    left: Option<Box<QuadBox>>,
    right: Option<Box<QuadBox>>,
    horizontal: bool,
}

impl QuadBox {
    fn weight<I: AABB>(&self, boxes: &[I]) -> Coord {
        let c: Coord = self
            .items
            .iter()
            .map(|i| &boxes[*i])
            .map(|b| b.middle())
            .sum();

        let si = self.items.len() as f64;
        Coord {
            x: c.x / si,
            y: c.y / si,
        }
    }

    fn split<I: AABB + Send + Sync>(
        &mut self,
        boxes: &[I],
    ) -> Option<(QuadBox, QuadBox, Vec<usize>)> {
        if self.items.len() > 5 {
            let weight = self.weight(boxes);

            let (left, right, intersection) = if !self.horizontal {
                let mut left = Vec::new();
                let mut right = Vec::new();
                let mut intersection = Vec::new();
                // left or right
                for i in &self.items {
                    let bo = &boxes[*i];
                    let x_min = bo.min().x;
                    let x_max = bo.max().x;

                    if x_max < weight.x {
                        left.push(*i);
                    } else if x_min > weight.x {
                        right.push(*i);
                    } else {
                        intersection.push(*i);
                    }
                }

                let left = QuadBox {
                    aabb: BasicAABB::new(
                        self.aabb.min(),
                        Coord {
                            x: weight.x,
                            y: self.aabb.max().x,
                        },
                    ),
                    items: left,
                    left: None,
                    right: None,
                    horizontal: true,
                    intersection: Vec::new(),
                };
                let right = QuadBox {
                    aabb: BasicAABB::new(
                        Coord {
                            x: weight.x,
                            y: self.aabb.min().y,
                        },
                        self.aabb.max(),
                    ),
                    items: right,
                    left: None,
                    right: None,
                    horizontal: true,
                    intersection: Vec::new(),
                };

                (left, right, intersection)
            } else {
                let mut up = Vec::new();
                let mut down = Vec::new();
                let mut intersection = Vec::new();
                // up or down
                for i in &self.items {
                    let bo = &boxes[*i];
                    let y_min = bo.min().y;
                    let y_max = bo.max().y;

                    if y_max < weight.y {
                        up.push(*i);
                    } else if y_min > weight.y {
                        down.push(*i);
                    } else {
                        intersection.push(*i);
                    }
                }

                let up = QuadBox {
                    aabb: BasicAABB::new(
                        self.aabb.min(),
                        Coord {
                            x: self.aabb.max().x,
                            y: weight.y,
                        },
                    ),
                    items: up,
                    left: None,
                    right: None,
                    horizontal: true,
                    intersection: Vec::new(),
                };
                let down = QuadBox {
                    aabb: BasicAABB::new(
                        Coord {
                            x: self.aabb.min().x,
                            y: weight.y,
                        },
                        self.aabb.max(),
                    ),
                    items: down,
                    left: None,
                    right: None,
                    horizontal: true,
                    intersection: Vec::new(),
                };

                (up, down, intersection)
            };

            Some((left, right, intersection))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::BasicAABB;
    use crate::Coord;

    use super::QuadTree;

    #[test]
    fn test1() {
        let mut m = QuadTree::new(get_boxes());
        m.calculate();

        println!("{:?}", m);
    }

    fn get_boxes() -> Vec<BasicAABB> {
        vec![
            BasicAABB::new(Coord { x: -2f64, y: -3f64 }, Coord { x: 2f64, y: 0f64 }),
            BasicAABB::new(Coord { x: -1f64, y: -2f64 }, Coord { x: 2f64, y: 0f64 }),
            BasicAABB::new(
                Coord {
                    x: -1f64,
                    y: -43f64,
                },
                Coord { x: 1f64, y: 0f64 },
            ),
            BasicAABB::new(
                Coord {
                    x: -20f64,
                    y: -12f64,
                },
                Coord { x: 2f64, y: 0f64 },
            ),
            BasicAABB::new(
                Coord {
                    x: -10f64,
                    y: -13f64,
                },
                Coord { x: 3f64, y: 0f64 },
            ),
            BasicAABB::new(
                Coord {
                    x: -43f64,
                    y: -43f64,
                },
                Coord { x: 4f64, y: 0f64 },
            ),
            BasicAABB::new(
                Coord {
                    x: -12f64,
                    y: -12f64,
                },
                Coord { x: 5f64, y: 0f64 },
            ),
            BasicAABB::new(
                Coord {
                    x: -32f64,
                    y: -6f64,
                },
                Coord { x: 6f64, y: 0f64 },
            ),
            // BasicAABB::new(Coord {x: -12f64, y: -2f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -54f64, y: -2f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -76f64, y: -12f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -44f64, y: -6f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -8f64, y: -9f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -1f64, y: -30f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -12f64, y: -14f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -32f64, y: -54f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -54f64, y: -23f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -22f64, y: -43f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -14f64, y: -54f64}, Coord {x: 2f64, y: 0f64}),
            // BasicAABB::new(Coord {x: -71f64, y: -65f64}, Coord {x: 2f64, y: 0f64}),
        ]
    }
}
