use kero::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SpritePatch(Rc<Inner>);

#[derive(Debug)]
struct Inner {
    texture: Texture,
    left_w: f32,
    right_w: f32,
    top_h: f32,
    bottom_h: f32,
    tx: [f32; 4],
    ty: [f32; 4],
}

impl SpritePatch {
    pub fn new(texture: Texture, outer: RectF, inner: RectF) -> Self {
        let x = [outer.x, inner.x, inner.right(), outer.right()];
        let y = [outer.y, inner.y, inner.bottom(), outer.bottom()];

        let left_w = x[1] - x[0];
        let right_w = x[3] - x[2];
        let top_h = y[1] - y[0];
        let bottom_h = y[3] - y[2];

        let size = texture.size().to_f32();
        let tx = x.map(|x| x / size.x);
        let ty = y.map(|y| y / size.y);

        Self(Rc::new(Inner {
            texture,
            left_w,
            right_w,
            top_h,
            bottom_h,
            tx,
            ty,
        }))
    }

    #[inline]
    pub fn texture(&self) -> &Texture {
        &self.0.texture
    }

    pub fn draw_ext(&mut self, draw: &mut Draw, rect: RectF, color: Rgba8, mode: ColorMode) {
        let px = [
            rect.x,
            rect.x + self.0.left_w,
            rect.right() - self.0.right_w,
            rect.right(),
        ];
        let py = [
            rect.y,
            rect.y + self.0.top_h,
            rect.bottom() - self.0.bottom_h,
            rect.bottom(),
        ];
        let vert = |i, j| {
            Vertex::new(
                vec2(px[i], py[j]),
                vec2(self.0.tx[i], self.0.ty[j]),
                color,
                mode,
            )
        };
        draw.custom(
            Some(self.texture().clone()),
            Topology::Triangles,
            [
                vert(0, 0),
                vert(1, 0),
                vert(2, 0),
                vert(3, 0),
                vert(0, 1),
                vert(1, 1),
                vert(2, 1),
                vert(3, 1),
                vert(0, 2),
                vert(1, 2),
                vert(2, 2),
                vert(3, 2),
                vert(0, 3),
                vert(1, 3),
                vert(2, 3),
                vert(3, 3),
            ],
            [
                0, 1, 5, 0, 5, 4, 1, 2, 6, 1, 6, 5, 2, 3, 7, 2, 7, 6, 4, 5, 9, 4, 9, 8, 5, 6, 10,
                5, 10, 9, 6, 7, 11, 6, 11, 10, 8, 9, 13, 8, 13, 12, 9, 10, 14, 9, 14, 13, 10, 11,
                15, 10, 15, 14,
            ],
        );
    }

    #[inline]
    pub fn draw(&mut self, draw: &mut Draw, rect: RectF) {
        self.draw_ext(draw, rect, Rgba8::WHITE, ColorMode::MULT);
    }
}
