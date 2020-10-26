pub fn plot_brezenham<F : FnMut(i32, i32) -> ()>(x0: i32, y0: i32, x1: i32, y1: i32, mut plot_func: F) {
    if y0 == y1 {
        for x in x0.min(x1)..=x0.max(x1) { plot_func(x, y0); }
    } else if x0 == x1 {
        for y in y0.min(y1)..=y0.max(y1) { plot_func(x0, y); }
    } else {
        let (dx_abs, dy_abs) = ((x1 - x0).abs(), (y1 - y0).abs());
        let (dx2, dy2) = (dx_abs * 2, dy_abs * 2);
        if dx_abs >= dy_abs {
            let (x0, x1, y0, y1) = if x0 > x1 {
                (x1, x0, y1, y0)
            } else {
                (x0, x1, y0, y1)
            };
            let sign = if y0 < y1 { 1 } else { -1};
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
            let sign = if x0 < x1 { 1 } else { -1};
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