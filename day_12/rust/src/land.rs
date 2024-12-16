use std::{
    io::{self, stdin, stdout, Read, Write},
    path::Display,
    process::Command,
};

use crate::{display_map::DisplayMap, plot::Plot};

pub struct Land {
    land: Vec<Vec<char>>,
    pub plots: Vec<Plot>,
}

impl Land {
    pub fn new(land: Vec<Vec<char>>) -> Self {
        Self {
            land,
            plots: Vec::new(),
        }
    }

    pub fn calculate_fencing(&self) -> usize {
        self.plots.iter().map(|plot| plot.calculate_fencing()).sum()
    }

    pub fn calculate_fencing_sides(&self) -> usize {
        self.plots
            .iter()
            .map(|plot| plot.calculate_fencing_sides())
            .sum()
    }

    pub fn map_plots(&mut self) {
        let mut mapped_land = self.land.clone();

        loop {
            if let Some(plot_info) = mapped_land.iter().enumerate().find_map(|(y, line)| {
                if let Some((x, plot)) = line
                    .iter()
                    .enumerate()
                    .find(|(_, plot_id)| plot_id != &&'#')
                {
                    return Some((*plot, x, y));
                }
                return None;
            }) {
                self.plots.push(Land::map_plot(&mut mapped_land, plot_info));
            } else {
                break;
            }

            // DisplayMap::display(&mapped_land);

            // let mut stdout = stdout();
            // stdout.write(b"Press Enter to continue...").unwrap();
            // stdout.flush().unwrap();
            // stdin().read(&mut [0]).unwrap();
        }
    }

    fn map_plot(mapped_land: &mut Vec<Vec<char>>, plot_info: (char, usize, usize)) -> Plot {
        let mut plot_pos = vec![(plot_info.1, plot_info.2)];
        mapped_land[plot_info.2][plot_info.1] = '#';
        let mut current_index = 0;

        loop {
            if current_index == plot_pos.len() {
                break;
            }
            let pos = plot_pos.get(current_index).unwrap().clone();

            Land::check_surroundings(mapped_land, &mut plot_pos, &pos, plot_info.0);

            current_index += 1;
        }

        Plot::new(plot_info.0, plot_pos)
    }

    fn check_surroundings(
        mapped_land: &mut Vec<Vec<char>>,
        plot_pos: &mut Vec<(usize, usize)>,
        current_pos: &(usize, usize),
        plot_id: char,
    ) {
        if current_pos.0 > 0 && mapped_land[current_pos.1][current_pos.0 - 1] == plot_id {
            mapped_land[current_pos.1][current_pos.0 - 1] = '#';
            plot_pos.push((current_pos.0 - 1, current_pos.1));
        }
        if current_pos.0 < mapped_land[0].len() - 1
            && mapped_land[current_pos.1][current_pos.0 + 1] == plot_id
        {
            mapped_land[current_pos.1][current_pos.0 + 1] = '#';
            plot_pos.push((current_pos.0 + 1, current_pos.1));
        }

        if current_pos.1 > 0 && mapped_land[current_pos.1 - 1][current_pos.0] == plot_id {
            mapped_land[current_pos.1 - 1][current_pos.0] = '#';
            plot_pos.push((current_pos.0, current_pos.1 - 1));
        }
        if current_pos.1 < mapped_land.len() - 1
            && mapped_land[current_pos.1 + 1][current_pos.0] == plot_id
        {
            mapped_land[current_pos.1 + 1][current_pos.0] = '#';
            plot_pos.push((current_pos.0, current_pos.1 + 1));
        }
    }
}
