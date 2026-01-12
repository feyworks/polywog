---@meta

---@alias BlendMode "normal"|"add"|"subtract"|"multiply"
---@alias Topology "triangles"|"lines"|"points"

---@class DrawModule
local Draw = {}

---Set the target surface and optionally clear it with a single color. If `None` is passed
---as the surface, the window will be drawn to. If `None` is passed as the clear color, then
---the surface will not be cleared, drawing will instead be appended to its current pixels.
---@param surface Surface?
---@param clear_color Color?
function Draw.set_surface(surface, clear_color) end

---Set the target layer. For the most part you will be rendering to the default layer `0`
---but in rare cases you may want to use layers to improve render batching.
---@param layer integer
function Draw.set_layer(layer) end

---Set the shader future drawing methods will use. If the shader is already in use, nothing
---will happen. If not, the shader will switch and all the new shader's parameters will be
---initialized with their default values.Methods
---@param shader Shader?
function Draw.set_shader(shader) end

---Set an `i32` parameter.
---@param name string
---@param value integer
function Draw.set_param_i32(name, value) end

---Set a `u32` parameter.
---@param name string
---@param value integer
function Draw.set_param_u32(name, value) end

---Set an `f32` parameter.
---@param name string
---@param value number
function Draw.set_param_f32(name, value) end

---Set a `vec2f` parameter.
---@param name string
---@param value Vec2
function Draw.set_param_vec2(name, value) end

---Set a `vec3f` parameter.
---@param name string
---@param value Vec3
function Draw.set_param_vec3(name, value) end

---Set a `vec4f` parameter.
---@param name string
---@param value Vec4
function Draw.set_param_vec4(name, value) end

---Set a `mat2f` parameter.
---@param name string
---@param value Mat2
function Draw.set_param_mat2(name, value) end

---Set a `mat3f` parameter.
---@param name string
---@param value Mat3
function Draw.set_param_mat3(name, value) end

---Set a `mat4f` parameter.
---@param name string
---@param value Mat4
function Draw.set_param_mat4(name, value) end

---Set a `texture_2d<f32>` parameter.
---@param name string
---@param value Texture
function Draw.set_param_texture(name, value) end

---Set a `sampler` parameter.
---@param name string
---@param value Sampler
function Draw.set_param_sampler(name, value) end

---Set the view matrix.
---@param value Mat4
function Draw.set_view_matrix(value) end

---The current main sampler.
---@return Sampler
---@nodiscard
function Draw.main_sampler(self) end

---Set the main sampler.
---@param value Sampler
function Draw.set_main_sampler(value) end

---The current blend mode.
---@return Sampler
---@nodiscard
function Draw.blend_mode(self) end

---Set the blend mode.
---@param value BlendMode
function Draw.set_blend_mode(value) end

---The current clip rectangle.
---@return Rect
---@nodiscard
function Draw.clip_rect(self) end

---Set the clip rectangle.
---@param value Rect
function Draw.set_clip_rect(value) end

---The current transform.
---@return Affine2
---@nodiscard
function Draw.transform(self) end

---Concatenate and push a transform to the stack.
---@param matrix Affine2
function Draw.push_transform(matrix) end

---Push a new transform value to the top of the stack.
---@param matrix Affine2
function Draw.push_new_transform(matrix) end

---Set the value of the top transform.
---@param matrix Affine2
function Draw.set_transform(matrix) end

---Concatenate and push a translation matrix.
---@param amount Vec2
function Draw.push_translation(amount) end

---Concatenate and push a rotation matrix.
---@param radians number
function Draw.push_rotation(radians) end

---Concatenate and push a scaling matrix.
---@param scale Vec2|number
function Draw.push_scale(scale) end

---Concatenate and push a translation/rotation/scaling matrix.
---@param translation Vec2
---@param rotation number
---@param scale Vec2|number
function Draw.push_trs(translation, rotation, scale) end

---Pop a transform off the top of the stack.
function Draw.pop_transform(self) end

---Pop a number of transforms off the top of the stack.
---@param count integer
function Draw.pop_transforms(count) end

---Draw a quad filled with a texture.
---@param texture Texture
---@param quad Quad
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function Draw.textured_quad(texture, quad, color, mode, flip_x, flip_y) end

---Draw a texture with the top-left at the provided position.
---@param texture Texture
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function Draw.texture_at(texture, pos, color, mode, flip_x, flip_y) end

---Draw a single point.
---@param point Vec2
---@param color Color
function Draw.point(point, color) end

---Draw a single point.
---@param x number
---@param y number
---@param color Color
function Draw.point(x, y, color) end

---Draw a set of points.
---@param points Vec2[]
---@param color Color
function Draw.points(points, color) end

---Draw a line.
---@param x1 number
---@param y1 number
---@param x2 number
---@param y2 number
---@param color Color
function Draw.line(x1, y1, x2, y2, color) end

---Draw a line.
---@param from Vec2
---@param to Vec2
---@param color Color
function Draw.line(from, to, color) end

---Draw a line.
---@param line Line
---@param color Color
function Draw.line_obj(line, color) end

---Draw lines connecting the series of points into a chain, optionally looping to the start.
---@param points Vec2[]
---@param color Color
---@param loops boolean
function Draw.lines(points, color, loops) end

---Draw a filled triangle.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param color Color
function Draw.triangle(a, b, c, color) end

---Draw a filled triangle.
---@param tri Triangle
---@param color Color
function Draw.triangle_obj(tri, color) end

---Draw a triangle outline.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param color Color
function Draw.triangle_outline(a, b, c, color) end

---Draw a triangle outline.
---@param tri Triangle
---@param color Color
function Draw.triangle_obj_outline(tri, color) end

---Draw a filled quad.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param d Vec2
---@param color Color
function Draw.quad(a, b, c, d, color) end

---Draw a filled quad.
---@param quad Quad
---@param color Color
function Draw.quad_obj(quad, color) end

---Draw a quad outline.
---@param a Vec2
---@param b Vec2
---@param c Vec2
---@param d Vec2
---@param color Color
function Draw.quad_outline(a, b, c, d, color) end

---Draw a quad outline.
---@param quad Quad
---@param color Color
function Draw.quad_obj_outline(quad, color) end

---Draw a filled rectangle.
---@param x number
---@param y number
---@param w number
---@param h number
---@param color Color
function Draw.rect(x, y, w, h, color) end

---Draw a filled rectangle.
---@param rect Rect
---@param color Color
function Draw.rect_obj(rect, color) end

---Draw a rectangle outline.
---@param x number
---@param y number
---@param w number
---@param h number
---@param color Color
function Draw.rect_outline(x, y, w, h, color) end

---Draw a rectangle outline.
---@param rect Rect
---@param color Color
function Draw.rect_obj_outline(rect, color) end

---Draw a filled polygon.
---@param poly Polygon
---@param color Color
function Draw.polygon(poly, color) end

---Draw a polygon outline.
---@param poly Polygon
---@param color Color
function Draw.polygon_outline(poly, color) end

---Draw a filled circle.
---@param x number
---@param y number
---@param radius number
---@param color Color
---@param seg_count integer?
function Draw.circle(x, y, radius, color, seg_count) end

---Draw a filled circle.
---@param center Vec2
---@param radius number
---@param color Color
---@param seg_count integer?
function Draw.circle(center, radius, color, seg_count) end

---Draw a filled circle.
---@param circ Circle
---@param color Color
---@param seg_count integer?
function Draw.circle_obj(circ, color, seg_count) end

---Draw a circle outline.
---@param x number
---@param y number
---@param radius number
---@param color Color
---@param seg_count integer?
function Draw.circle_outline(x, y, radius, color, seg_count) end

---Draw a circle outline.
---@param center Vec2
---@param radius number
---@param color Color
---@param seg_count integer?
function Draw.circle_outline(center, radius, color, seg_count) end

---Draw a circle outline.
---@param circ Circle
---@param color Color
---@param seg_count integer?
function Draw.circle_obj_outline(circ, color, seg_count) end

---Draw a subtexture.
---@param sub SubTexture
---@param dst Quad
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function Draw.subtextured_quad(sub, dst, color, mode, flip_x, flip_y) end

---Draw a subtexture.
---@param sub SubTexture
---@param pos Vec2
---@param color Color?
---@param mode ColorMode?
---@param flip_x boolean?
---@param flip_y boolean?
function Draw.subtexture_at(sub, pos, color, mode, flip_x, flip_y) end

---Draw text with the provided font and size.Methods
---@param text string
---@param pos Vec2
---@param font Font
---@param color Color?
---@param size number?
function Draw.text(text, pos, font, size, color) end

---Draw text with the provided font and size.Methods
---@param text string
---@param x number
---@param y number
---@param font Font
---@param color Color?
---@param size number?
function Draw.text(text, x, y, font, size, color) end

---Draw a custom set of vertices & indices.
---@param texture Texture?
---@param topology Topology
---@param vertices Vertex[]
---@param indices integer[]
function Draw.custom(texture, topology, vertices, indices) end

---Draw the provided vertex & index buffers.
---@param texture Texture?
---@param topology Topology
---@param vertices VertexBuffer
---@param indices IndexBuffer
function Draw.buffers(texture, topology, vertices, indices) end

return Draw