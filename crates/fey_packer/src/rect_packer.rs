use crate::{Item, Packed};
use fey_math::{RectU, Vec2U};

/// A rectangle packer.
pub struct RectPacker {
    /// The maximum rectangle size we can pack into.
    pub max_size: u32,

    /// If the resulting size should be power-of-2 sized.
    pub power_of_two: bool,

    /// Padding to include around each item.
    pub padding: u32,

    /// Spacing to include between items.
    pub spacing: u32,
}

impl Default for RectPacker {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl RectPacker {
    /// Create default settings:
    /// - `max_size = 4096`
    /// - `power_of_two = true`
    /// - `padding = 0`
    /// - `spacing = 0`
    pub const fn new() -> Self {
        Self {
            max_size: 4096,
            power_of_two: true,
            padding: 0,
            spacing: 0,
        }
    }

    /// Set the maximum rectangle size we can pack into.
    #[inline]
    pub const fn with_max_size(mut self, max_size: u32) -> Self {
        self.max_size = max_size;
        self
    }

    /// Set if the resulting size should be power-of-2 sized.
    pub const fn with_power_of_two(mut self) -> Self {
        self.power_of_two = true;
        self
    }

    /// Set the padding to include around each item.
    pub const fn with_padding(mut self, padding: u32) -> Self {
        self.padding = padding;
        self
    }

    /// Set the spacing to include between items.
    pub const fn with_spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Pack a collection of rectangles.
    ///
    /// On success, this function will return a list of all the packed
    /// items and their pack locations, and the size of the rectangle
    /// that they were all able to fit in.
    pub fn pack<T>(&self, mut items: Vec<Item<T>>) -> Option<(Vec2U, Vec<Packed<T>>)> {
        // sort the items by height before packing
        items.sort_by_key(|item| item.size.x.max(item.size.x));

        let mut packed = Vec::new();
        let mut nodes = Vec::new();
        let extra = Vec2U::splat(self.padding * 2 + self.spacing);

        // fetch the largest item to pack
        if let Some(largest) = items.last() {
            // if the largest item is larger than our max size, don't bother packing
            if largest.size.x + self.padding * 2 > self.max_size
                || largest.size.y + self.padding * 2 > self.max_size
            {
                return None;
            }

            // if it will fit, make the root node
            packed.reserve(items.len());
            nodes.reserve(items.len() * 3);
            nodes.push(Node::new(RectU::sized(largest.size + extra)));
        } else {
            // if we have no items to pack, return successfully
            return Some((Vec2U::ZERO, packed));
        }

        let mut root: usize = 0;

        fn new_node(nodes: &mut Vec<Node>, x: u32, y: u32, w: u32, h: u32) -> usize {
            let i = nodes.len();
            nodes.push(Node::new(RectU::new(x, y, w, h)));
            i
        }

        fn find(nodes: &[Node], i: usize, size: &Vec2U) -> Option<usize> {
            let node = &nodes[i];
            if node.used {
                if let Some(right) = node.right {
                    if let Some(n) = find(nodes, right, size) {
                        return Some(n);
                    }
                }
                node.down.and_then(|down| find(nodes, down, size))
            } else {
                (size.x <= node.rect.w && size.y <= node.rect.h).then_some(i)
            }
        }

        while let Some(item) = items.pop() {
            let size = item.size + extra;

            let node = match find(&nodes, root, &size) {
                Some(node) => node,
                None => {
                    let root_rect = nodes[root].rect;

                    let can_grow_d = size.x <= root_rect.w && root_rect.h + size.y < self.max_size;
                    let can_grow_r = size.y <= root_rect.h && root_rect.w + size.x < self.max_size;
                    if !can_grow_d && !can_grow_r {
                        return None;
                    }

                    let should_grow_r = can_grow_r && root_rect.h >= root_rect.w + size.x;
                    let should_grow_d = can_grow_d && root_rect.w >= root_rect.h + size.y;

                    if should_grow_r || (!should_grow_d && can_grow_r) {
                        let next = new_node(&mut nodes, 0, 0, root_rect.w + size.x, root_rect.h);
                        nodes[next].used = true;
                        nodes[next].down = Some(root);
                        let node = new_node(&mut nodes, root_rect.w, 0, size.x, root_rect.h);
                        nodes[next].right = Some(node);
                        root = next;
                        node
                    } else {
                        let next = new_node(&mut nodes, 0, 0, root_rect.w, root_rect.h + size.y);
                        nodes[next].used = true;
                        let node = new_node(&mut nodes, 0, root_rect.h, root_rect.w, size.y);
                        nodes[next].down = Some(node);
                        nodes[next].right = Some(root);
                        root = next;
                        node
                    }
                }
            };

            let node_rect = nodes[node].rect;
            nodes[node].used = true;
            nodes[node].down = Some(new_node(
                &mut nodes,
                node_rect.x,
                node_rect.y + size.y,
                node_rect.w,
                node_rect.h - size.y,
            ));
            nodes[node].right = Some(new_node(
                &mut nodes,
                node_rect.x + size.x,
                node_rect.y,
                node_rect.w - size.x,
                size.y,
            ));

            packed.push(Packed {
                data: item.data,
                pos: node_rect.top_left() + Vec2U::splat(self.padding),
            });
        }

        let size = if self.power_of_two {
            nodes[root].rect.size().map(|x| x.next_power_of_two())
        } else {
            nodes[root].rect.size()
        };

        Some((size, packed))
    }
}

struct Node {
    used: bool,
    rect: RectU,
    right: Option<usize>,
    down: Option<usize>,
}

impl Node {
    pub const fn new(rect: RectU) -> Self {
        Self {
            used: false,
            rect,
            right: None,
            down: None,
        }
    }
}
