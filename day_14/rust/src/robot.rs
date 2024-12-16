pub struct Robot {
    starting_pos: (isize, isize),
    x_vel: isize,
    y_vel: isize,
    x_length: isize,
    y_length: isize,
}

impl Robot {
    pub fn new(
        starting_pos: (isize, isize),
        x_vel: isize,
        y_vel: isize,
        x_length: isize,
        y_length: isize,
    ) -> Self {
        Self {
            starting_pos,
            x_vel,
            y_vel,
            x_length,
            y_length,
        }
    }

    pub fn pos_after(&self, nr_iterations: isize) -> (isize, isize) {
        let x = Robot::get_axis_after(
            nr_iterations,
            self.x_length,
            self.starting_pos.0,
            self.x_vel,
        );
        let y = Robot::get_axis_after(
            nr_iterations,
            self.y_length,
            self.starting_pos.1,
            self.y_vel,
        );

        (x, y)
    }

    fn get_axis_after(
        nr_iterations: isize,
        axis_length: isize,
        start_pos: isize,
        vel: isize,
    ) -> isize {
        let vel_iter = (vel * nr_iterations).abs();
        if vel < 0 {
            let iter = (axis_length - 1 - start_pos) + vel_iter;
            let result = iter - ((iter / axis_length) * axis_length);
            return axis_length - 1 - result;
        }

        let iter = start_pos + vel_iter;
        return iter - ((iter / axis_length) * axis_length);
    }
}
