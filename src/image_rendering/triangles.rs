use std::mem::swap;
use crate::image_rendering::bresenham::{plot_bresenham_4d, plot_bresenham_3d};

pub fn draw_brezenham_triangles(
    bump: &bumpalo::Bump,
    verts: &[(
        [(i32, i32); 3], // position
        [(i32, i32); 3]  // uv
    )],
    w: usize, max_offset: usize,
    mut plot_func: impl FnMut(usize, (i32, i32))
) {
    let triangles = verts;
    for verts in triangles {
        let ((
            (v0x, v0y),
            (v1x, v1y),
            (v2x, v2y)
        ), (
            (v0u, v0v),
            (v1u, v1v),
            (v2u, v2v)
        )) = get_sorted(verts);

        let height_diff = (v2y - v0y) as usize;
        if height_diff == 0 { return; }

        let mut left_side = bumpalo::collections::Vec::new_in(bump);
        left_side.resize(height_diff + 1, [100_000, 0, 0]);
        let mut right_side = bumpalo::collections::Vec::new_in(bump);
        right_side.resize(height_diff + 1, [-100_000, 0, 0]);

        let mut dest_offset = v0y * w as i32;

        let t = (v1y - v0y) as f32 / height_diff as f32;
        let v3x = (v0x as f32 + t * (v2x - v0x) as f32) as i32;
        let midp_is_on_left = v3x >= v1x;

        if midp_is_on_left {
            plot_bresenham_4d(
                v0x, 0, v0u, v0v,
                v1x, v1y - v0y, v1u, v1v,
                |x, y, u, v| {
                    if x < left_side[y as usize][0] {
                        left_side[y as usize] = [x, u, v];
                    }
                }
            );
            plot_bresenham_4d(
                v1x, v1y - v0y, v1u, v1v,
                v2x, v2y - v0y, v2u, v2v,
                |x, y, u, v| {
                    if x < left_side[y as usize][0] {
                        left_side[y as usize] = [x, u, v];
                    }
                }
            );
            plot_bresenham_4d(
                v0x, 0, v0u, v0v,
                v2x, v2y - v0y, v2u, v2v,
                |x, y, u, v| {
                    if x > right_side[y as usize][0] {
                        right_side[y as usize] = [x, u, v];
                    }
                }
            );
        } else {
            plot_bresenham_4d(
                v0x, 0, v0u, v0v,
                v1x, v1y - v0y, v1u, v1v,
                |x, y, u, v| {
                    if x > right_side[y as usize][0] {
                        right_side[y as usize] = [x, u, v];
                    }
                }
            );
            plot_bresenham_4d(
                v1x, v1y - v0y, v1u, v1v,
                v2x, v2y - v0y, v2u, v2v,
                |x, y, u, v| {
                    if x > right_side[y as usize][0] {
                        right_side[y as usize] = [x, u, v];
                    }
                }
            );
            plot_bresenham_4d(
                v0x, 0, v0u, v0v,
                v2x, v2y - v0y, v2u, v2v,
                |x, y, u, v| {
                    if x < left_side[y as usize][0] {
                        left_side[y as usize] = [x, u, v];
                    }
                }
            );
        }

        let max_offset = max_offset as i32;
        for i in 0..=height_diff {
            if dest_offset >= max_offset {
                continue;
            }
            if dest_offset >= 0 {
                plot_bresenham_3d(
                    0, left_side[i][1], left_side[i][2],
                    right_side[i][0] - left_side[i][0], right_side[i][1], right_side[i][2],
                    |ix, u, v| {
                        let x = left_side[i][0] + ix;
                        if x >= 0 && x < w as i32 {
                            plot_func((dest_offset + x) as usize, (u, v));
                        }
                    }
                );
            }
            dest_offset += w as i32;
        }
    }

    fn get_sorted(verts: &(
        [(i32, i32); 3], // position
        [(i32, i32); 3]  // uv
    )) -> (
        ((i32, i32), (i32, i32), (i32, i32)), // position
        ((i32, i32), (i32, i32), (i32, i32))  // uv
    ) {
        let (mut p0, mut p1, mut p2) = (verts.0[0], verts.0[1], verts.0[2]);
        let (p0, p1, p2) = (&mut p0, &mut p1, &mut p2);
        let (mut uv0, mut uv1, mut uv2) = (verts.1[0], verts.1[1], verts.1[2]);
        let (uv0, uv1, uv2) = (&mut uv0, &mut uv1, &mut uv2);

        if p0.1 > p1.1 { swap(p0, p1); swap(uv0, uv1) }
        if p0.1 > p2.1 { swap(p0, p2); swap(uv0, uv2) }
        if p1.1 > p2.1 { swap(p1, p2); swap(uv1, uv2) }
        let (p0, p1, p2) = (*p0, *p1, *p2);
        let (uv0, uv1, uv2) = (*uv0, *uv1, *uv2);
        ((p0, p1, p2), (uv0, uv1, uv2))
    }
}