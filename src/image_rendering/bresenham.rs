pub fn plot_bresenham_circle(
    cx: i32, cy: i32, r: i32,
    mut plot_func: impl FnMut(i32, i32) -> ()
) {
    fn plot_all_octants(cx: i32, cy: i32, dx: i32, dy: i32, plot_func: &mut impl FnMut(i32, i32) -> ()) {
        let (x0, x1, x2, x3, x4, x5, x6, x7) = (
            cx + dx, cx + dx, cx - dx, cx - dx, cx + dy, cx + dy, cx - dy, cx - dy
        );
        let (y0, y1, y2, y3, y4, y5, y6, y7) = (
            cy + dy, cy - dy, cy + dy, cy - dy, cy + dx, cy - dx, cy + dx, cy - dx
        );
        plot_func(x0, y0);
        plot_func(x1, y1);
        plot_func(x2, y2);
        plot_func(x3, y3);
        plot_func(x4, y4);
        plot_func(x5, y5);
        plot_func(x6, y6);
        plot_func(x7, y7);
    }
    let mut d = 3 - r * 2;
    let mut x = 0;
    let mut y = r;
    plot_all_octants(cx, cy, x, y, &mut plot_func);
    while x < y {
        if d <= 0 {
            d += 6 + (x << 2);
        } else {
            d += 10 + ((x - y) << 2);
            y -= 1;
        }
        x += 1;
        plot_all_octants(cx, cy, x, y, &mut plot_func);
    }
}

pub fn plot_bresenham_2d<F : FnMut(i32, i32) -> ()>(x0: i32, y0: i32, x1: i32, y1: i32, mut plot_func: F) {
    if y0 == y1 {
        for x in x0.min(x1)..=x0.max(x1) { plot_func(x, y0); }
    } else if x0 == x1 {
        for y in y0.min(y1)..=y0.max(y1) { plot_func(x0, y); }
    } else {
        let (dx_abs, dy_abs) = ((x1 - x0).abs(), (y1 - y0).abs());
        let (dx2, dy2) = (dx_abs << 1, dy_abs << 1);
        if dx_abs >= dy_abs {
            let (x0, x1, y0, y1) = if x0 > x1 {
                (x1, x0, y1, y0)
            } else {
                (x0, x1, y0, y1)
            };
            let sign = if y0 < y1 { 1 } else { -1 };
            let mut y = y0;
            let mut d = dy2 - dx_abs;
            for x in x0..=x1 {
                plot_func(x, y);
                if d > 0 {
                    d -= dx2;
                    y += sign;
                }
                d += dy2;
            }
        } else {
            let (x0, x1, y0, y1) = if y0 > y1 {
                (x1, x0, y1, y0)
            } else {
                (x0, x1, y0, y1)
            };
            let sign = if x0 < x1 { 1 } else { -1 };
            let mut x = x0;
            let mut d = dx2 - dy_abs;
            for y in y0..=y1 {
                plot_func(x, y);
                if d > 0 {
                    d -= dy2;
                    x += sign;
                }
                d += dx2;
            }
        }
    }
}

pub fn plot_bresenham_3d<F : FnMut(i32, i32, i32) -> ()>(
    x0: i32, y0: i32, z0: i32,
    x1: i32, y1: i32, z1: i32,
    mut plot_func: F
) {
    let (dx_abs, dy_abs, dz_abs) = (
        (x1 - x0).abs(),
        (y1 - y0).abs(),
        (z1 - z0).abs()
    );
    let (dx2, dy2, dz2) = (dx_abs << 1, dy_abs << 1, dz_abs << 1);
    if dx_abs >= dy_abs && dx_abs >= dz_abs {
        let (x0, x1, y0, y1, z0, z1) = if x0 > x1 {
            (x1, x0, y1, y0, z1, z0)
        } else {
            (x0, x1, y0, y1, z0, z1)
        };
        let sign_y = if y0 < y1 { 1 } else { -1 };
        let sign_z = if z0 < z1 { 1 } else { -1 };
        let (mut y, mut z) = (y0, z0);
        let mut d_y = dy2 - dx_abs;
        let mut d_z = dz2 - dx_abs;
        for x in x0..=x1 {
            plot_func(x, y, z);
            if d_y > 0 {
                d_y -= dx2;
                y += sign_y;
            }
            d_y += dy2;
            if d_z > 0 {
                d_z -= dx2;
                z += sign_z;
            }
            d_z += dz2;
        }
    } else if dy_abs >= dx_abs && dy_abs >= dz_abs {
        let (x0, x1, y0, y1, z0, z1) = if y0 > y1 {
            (x1, x0, y1, y0, z1, z0)
        } else {
            (x0, x1, y0, y1, z0, z1)
        };
        let sign_x = if x0 < x1 { 1 } else { -1 };
        let sign_z = if z0 < z1 { 1 } else { -1 };
        let (mut x, mut z) = (x0, z0);
        let mut d_x = dx2 - dy_abs;
        let mut d_z = dz2 - dy_abs;
        for y in y0..=y1 {
            plot_func(x, y, z);
            if d_x > 0 {
                d_x -= dy2;
                x += sign_x;
            }
            d_x += dx2;
            if d_z > 0 {
                d_z -= dy2;
                z += sign_z;
            }
            d_z += dz2;
        }
    } else {
        let (x0, x1, y0, y1, z0, z1) = if z0 > z1 {
            (x1, x0, y1, y0, z1, z0)
        } else {
            (x0, x1, y0, y1, z0, z1)
        };
        let sign_x = if x0 < x1 { 1 } else { -1 };
        let sign_y = if y0 < y1 { 1 } else { -1 };
        let (mut x, mut y) = (x0, y0);
        let mut d_x = dx2 - dz_abs;
        let mut d_y = dy2 - dz_abs;
        for z in z0..=z1 {
            plot_func(x, y, z);
            if d_x > 0 {
                d_x -= dz2;
                x += sign_x;
            }
            d_x += dx2;
            if d_y > 0 {
                d_y -= dz2;
                y += sign_y;
            }
            d_y += dy2;
        }
    }
}

pub fn plot_bresenham_4d<F : FnMut(i32, i32, i32, i32) -> ()>(
    x0: i32, y0: i32, z0: i32, w0: i32,
    x1: i32, y1: i32, z1: i32, w1: i32,
    mut plot_func: F
) {
    let (dx_abs, dy_abs, dz_abs, dw_abs) = (
        (x1 - x0).abs(),
        (y1 - y0).abs(),
        (z1 - z0).abs(),
        (w1 - w0).abs()
    );
    let (dx2, dy2, dz2, dw2) = (dx_abs << 1, dy_abs << 1, dz_abs << 1, dw_abs << 1);
    if dx_abs >= dy_abs && dx_abs >= dz_abs && dx_abs >= dw_abs {
        let (x0, x1, y0, y1, z0, z1, w0, w1) = if x0 > x1 {
            (x1, x0, y1, y0, z1, z0, w1, w0)
        } else {
            (x0, x1, y0, y1, z0, z1, w0, w1)
        };
        let sign_y = if y0 < y1 { 1 } else { -1 };
        let sign_z = if z0 < z1 { 1 } else { -1 };
        let sign_w = if w0 < w1 { 1 } else { -1 };
        let (mut y, mut z, mut w) = (y0, z0, w0);
        let mut d_y = dy2 - dx_abs;
        let mut d_z = dz2 - dx_abs;
        let mut d_w = dw2 - dx_abs;
        for x in x0..=x1 {
            plot_func(x, y, z, w);
            if d_y > 0 {
                d_y -= dx2;
                y += sign_y;
            }
            d_y += dy2;
            if d_z > 0 {
                d_z -= dx2;
                z += sign_z;
            }
            d_z += dz2;
            if d_w > 0 {
                d_w -= dx2;
                w += sign_w;
            }
            d_w += dw2;
        }
    } else if dy_abs >= dx_abs && dy_abs >= dz_abs && dy_abs >= dw_abs {
        let (x0, x1, y0, y1, z0, z1, w0, w1) = if y0 > y1 {
            (x1, x0, y1, y0, z1, z0, w1, w0)
        } else {
            (x0, x1, y0, y1, z0, z1, w0, w1)
        };
        let sign_x = if x0 < x1 { 1 } else { -1 };
        let sign_z = if z0 < z1 { 1 } else { -1 };
        let sign_w = if w0 < w1 { 1 } else { -1 };
        let (mut x, mut z, mut w) = (x0, z0, w0);
        let mut d_x = dx2 - dy_abs;
        let mut d_z = dz2 - dy_abs;
        let mut d_w = dw2 - dy_abs;
        for y in y0..=y1 {
            plot_func(x, y, z, w);
            if d_x > 0 {
                d_x -= dy2;
                x += sign_x;
            }
            d_x += dx2;
            if d_z > 0 {
                d_z -= dy2;
                z += sign_z;
            }
            d_z += dz2;
            if d_w > 0 {
                d_w -= dy2;
                w += sign_w;
            }
            d_w += dw2;
        }
    } else if dz_abs >= dx_abs && dz_abs >= dy_abs && dz_abs >= dw_abs {
        let (x0, x1, y0, y1, z0, z1, w0, w1) = if z0 > z1 {
            (x1, x0, y1, y0, z1, z0, w1, w0)
        } else {
            (x0, x1, y0, y1, z0, z1, w0, w1)
        };
        let sign_x = if x0 < x1 { 1 } else { -1 };
        let sign_y = if y0 < y1 { 1 } else { -1 };
        let sign_w = if w0 < w1 { 1 } else { -1 };
        let (mut x, mut y, mut w) = (x0, y0, w0);
        let mut d_x = dx2 - dz_abs;
        let mut d_y = dy2 - dz_abs;
        let mut d_w = dw2 - dz_abs;
        for z in z0..=z1 {
            plot_func(x, y, z, w);
            if d_x > 0 {
                d_x -= dz2;
                x += sign_x;
            }
            d_x += dx2;
            if d_y > 0 {
                d_y -= dz2;
                y += sign_y;
            }
            d_y += dy2;
            if d_w > 0 {
                d_w -= dz2;
                w += sign_w;
            }
            d_w += dw2;
        }
    } else {
        let (x0, x1, y0, y1, z0, z1, w0, w1) = if w0 > w1 {
            (x1, x0, y1, y0, z1, z0, w1, w0)
        } else {
            (x0, x1, y0, y1, z0, z1, w0, w1)
        };
        let sign_x = if x0 < x1 { 1 } else { -1 };
        let sign_y = if y0 < y1 { 1 } else { -1 };
        let sign_z = if z0 < z1 { 1 } else { -1 };
        let (mut x, mut y, mut z) = (x0, y0, z0);
        let mut d_x = dx2 - dw_abs;
        let mut d_y = dy2 - dw_abs;
        let mut d_z = dz2 - dw_abs;
        for w in w0..=w1 {
            plot_func(x, y, z, w);
            if d_x > 0 {
                d_x -= dw2;
                x += sign_x;
            }
            d_x += dx2;
            if d_y > 0 {
                d_y -= dw2;
                y += sign_y;
            }
            d_y += dy2;
            if d_z > 0 {
                d_z -= dw2;
                z += sign_z;
            }
            d_z += dz2;
        }
    }
}