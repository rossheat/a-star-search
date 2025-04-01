// https://github.com/tomaka/wasm-timer/issues/14
use fluvio_wasm_timer::Delay;
use rand::Rng;
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures;
use web_sys::{HtmlElement, console, window};

#[cfg(feature = "console_error_panic_hook")]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen(start)]
pub fn setup() -> Result<(), JsValue> {
    set_panic_hook();
    console::log_1(&"Successfully set_panic_hook".into());
    Ok(())
}

#[derive(Clone)]
struct Cell {
    pub vec_idx: usize,
    pub row: usize,
    pub col: usize,
    pub html_element: HtmlElement,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.vec_idx == other.vec_idx
    }
}

impl Cell {
    pub fn new(vec_idx: usize, html_element: HtmlElement, row: usize, col: usize) -> Cell {
        Cell {
            vec_idx,
            row,
            col,
            html_element,
        }
    }

    pub fn set_g_score(&mut self, g: usize) {
        self.html_element.set_attribute("g", &g.to_string());
    }

    pub fn get_g_score(&self) -> usize {
        self.html_element
            .get_attribute("g")
            .expect("g score must exist")
            .parse()
            .expect("g score must be a number")
    }

    pub fn set_h_score(&mut self, h: usize) {
        self.html_element.set_attribute("h", &h.to_string());
    }

    pub fn get_h_score(&self) -> usize {
        self.html_element
            .get_attribute("h")
            .expect("h score must exist")
            .parse()
            .expect("h score must be a number")
    }

    pub fn set_f_score(&mut self, f: usize) {
        self.html_element.set_attribute("f", &f.to_string());
    }

    pub fn get_f_score(&self) -> usize {
        self.html_element
            .get_attribute("f")
            .expect("f score must exist")
            .parse()
            .expect("f score must be a number")
    }

    pub fn set_parent_cell_idx(&mut self, parent_cell_idx: Option<usize>) {
        match parent_cell_idx {
            Some(idx) => {
                self.html_element
                    .set_attribute("parent_cell_idx", &idx.to_string());
            }
            None => {
                self.html_element.set_attribute("parent_cell_idx", "");
            }
        }
    }

    pub fn get_parent_cell_idx(&self) -> Option<usize> {
        let parent_cell_idx = self.html_element.get_attribute("parent_cell_idx");
        match parent_cell_idx {
            Some(idx) => {
                if idx == "" {
                    return None;
                }
                Some(idx.parse().expect("parent_cell_idx must be a number"))
            }
            None => None,
        }
    }
}

struct Grid {
    n_rows_and_cols: usize,
    pub cells: Vec<Cell>,
    pub source_cell_idx: usize,
    pub goal_cell_idx: usize,
    obstacle_indices: Vec<usize>,
}

impl Grid {
    pub fn new(n_rows_and_cols: usize) -> Result<Grid, JsValue> {
        let mut grid = Grid {
            n_rows_and_cols,
            cells: Vec::new(),
            source_cell_idx: 0,
            goal_cell_idx: 0,
            obstacle_indices: Vec::new(),
        };

        grid.cells = grid.make_cells()?;
        grid.source_cell_idx = grid.gen_rand_source_idx();
        grid.goal_cell_idx = grid.gen_rand_goal_idx();
        grid.obstacle_indices = grid.gen_rand_obstacle_indices();

        grid.draw()?;

        Ok(grid)
    }

    pub fn make_cells(&mut self) -> Result<Vec<Cell>, JsValue> {
        let mut cells = Vec::new();
        for row in 0..self.n_rows_and_cols {
            for col in 0..self.n_rows_and_cols {
                let window = window().expect("no global `window` exists");
                let document = window.document().expect("should have a document on window");
                let cell = document.create_element("div")?;
                let cell = cell.dyn_into::<HtmlElement>()?;
                let style = cell.style();
                cell.set_attribute("id", &format!("cell-{}-{}", row, col))?;
                cell.set_attribute("vec_idx", &(row * self.n_rows_and_cols + col).to_string())?;
                cell.set_attribute("row", &row.to_string())?;
                cell.set_attribute("col", &col.to_string())?;
                cell.set_attribute("width", "20px");
                cell.set_attribute("height", "20px");
                cell.set_class_name("cell");
                // cells.push(cell);
                cells.push(Cell::new(row * self.n_rows_and_cols + col, cell, row, col));
            }
        }
        Ok(cells)
    }

    fn gen_rand_obstacle_indices(&self) -> Vec<usize> {
        let n_obstacles = (self.n_rows_and_cols * self.n_rows_and_cols) / 2;
        let mut rng = rand::thread_rng();
        let mut indices: Vec<usize> = (0..n_obstacles)
            .map(|_| rng.gen_range(0..self.cells.len()))
            .collect();
        indices.retain(|&num| num != self.source_cell_idx && num != self.goal_cell_idx);
        indices
    }

    fn gen_rand_source_idx(&self) -> usize {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..self.cells.len())
    }

    fn gen_rand_goal_idx(&self) -> usize {
        let mut rng = rand::thread_rng();
        let mut idx = 0;
        loop {
            idx = rng.gen_range(0..self.cells.len());
            if idx != self.source_cell_idx {
                break;
            }
        }
        idx
    }

    pub fn draw(&self) -> Result<(), JsValue> {
        let window = window().expect("no global `window` exists");
        let document = window.document().expect("should have a document on window");
        let body = document.body().expect("document should have a body");

        let maybeContainer = document.get_element_by_id("container");
        match maybeContainer {
            Some(container) => {
                body.remove_child(&container)?;
            }
            None => {}
        }

        let style = body.style();
        style.set_property("display", "flex")?;
        style.set_property("justify-content", "center")?;
        style.set_property("align-items", "center")?;
        style.set_property("height", "100vh")?;
        style.set_property("margin", "0")?;

        let container = document.create_element("div")?;
        let container = container.dyn_into::<HtmlElement>()?;
        container.set_attribute("id", "container");
        let style = container.style();
        style.set_property("display", "flex")?;
        style.set_property("flexDirection", "column")?;

        let cell_size = format!("calc(min(90vw , 90vh) / {})", self.n_rows_and_cols);

        for r in 0..self.n_rows_and_cols {
            let row = document.create_element("div")?;
            let row = row.dyn_into::<HtmlElement>()?;
            let style = row.style();
            // style.set_property("display", "flex")?;

            for c in 0..self.n_rows_and_cols {
                let cell = self
                    .cells
                    .get(c * self.n_rows_and_cols + r)
                    .expect("Cell must exist");
                let style = cell.html_element.style();
                style.set_property("width", &cell_size)?;
                style.set_property("height", &cell_size)?;
                style.set_property("border", "1px solid gray")?;
                if cell == self.cells.get(self.source_cell_idx).expect("") {
                    // style.set_property("background-color", "blue")?;
                    style.set_property("background-color", "blue")?;
                    style.set_property("font-size", "30px")?;
                    style.set_property("display", "flex")?;
                    style.set_property("justify-content", "center")?;
                    style.set_property("align-items", "center")?;
                    style.set_property("color", "white")?;
                    cell.html_element.set_inner_html("S");
                } else if cell == self.cells.get(self.goal_cell_idx).expect("") {
                    style.set_property("background-color", "green")?;
                    cell.html_element.set_inner_html("G");
                    style.set_property("color", "white")?;
                    style.set_property("font-size", "30px")?;
                    style.set_property("display", "flex")?;
                    style.set_property("align-items", "center")?;
                    style.set_property("justify-content", "center")?;
                } else if self
                    .obstacle_indices
                    .contains(&(c * self.n_rows_and_cols + r))
                {
                    style.set_property("background-color", "black")?;
                } else {
                    style.set_property("background-color", "white")?;
                }
                row.append_child(&cell.html_element)?;
            }
            container.append_child(&row)?;
        }
        body.append_child(&container)?;

        Ok(())
    }

    fn reset(&mut self) -> Result<(), JsValue> {
        self.cells = self.make_cells()?;
        self.source_cell_idx = self.gen_rand_source_idx();
        self.goal_cell_idx = self.gen_rand_goal_idx();
        self.obstacle_indices = self.gen_rand_obstacle_indices();

        Ok(())
    }
}

struct AStar {
    grid: Grid,
}

impl AStar {
    fn get_cell_with_lowest_f_score(&self, open_list: &Vec<Cell>) -> Cell {
        let mut lowest_f_score = usize::MAX;
        let mut lowest_f_score_cell = open_list
            .get(0)
            .expect("Open list must have at least one cell")
            .clone();
        for cell in open_list {
            if cell.get_f_score() < lowest_f_score {
                lowest_f_score = cell.get_f_score();
                lowest_f_score_cell = cell.clone();
            }
        }
        lowest_f_score_cell
    }

    fn euclidean_distance(&self, x1: usize, x2: usize, y1: usize, y2: usize) -> usize {
        let x = x1 - x2;
        let y = y1 - y2;
        ((x * x + y * y) as f64).sqrt() as usize
    }

    fn get_neighbours(&self, cell: Cell) -> Vec<Cell> {
        let mut neighbours = vec![];
        let mut indices = vec![];

        if (cell.col != 0) {
            // top left
            indices.push(cell.vec_idx - self.grid.n_rows_and_cols - 1);
            // left
            indices.push(cell.vec_idx - 1);
            // bottom left
            indices.push(cell.vec_idx + self.grid.n_rows_and_cols - 1);
        }

        if (cell.col != self.grid.n_rows_and_cols - 1) {
            // top right
            indices.push(cell.vec_idx - self.grid.n_rows_and_cols + 1);
            // right
            indices.push(cell.vec_idx + 1);
            // bottom right
            indices.push(cell.vec_idx + self.grid.n_rows_and_cols + 1);
        }

        // up
        indices.push(cell.vec_idx - self.grid.n_rows_and_cols);
        // down
        indices.push(cell.vec_idx + self.grid.n_rows_and_cols);

        for idx in indices {
            if self.grid.obstacle_indices.contains(&idx) {
                continue;
            }

            let neighbour = self.grid.cells.get(idx);
            match neighbour {
                Some(neighbour) => {
                    neighbours.push(neighbour.clone());
                }
                None => {}
            }
        }

        return neighbours;
    }

    fn get_neighbour_indices(&self, cell_idx: usize) -> Vec<usize> {
        let mut indices = vec![];

        let cell = &self.grid.cells[cell_idx];

        if cell.col != 0 {
            indices.push(cell_idx - self.grid.n_rows_and_cols - 1);
            indices.push(cell_idx - 1);
            indices.push(cell_idx + self.grid.n_rows_and_cols - 1);
        }

        if cell.col != self.grid.n_rows_and_cols - 1 {
            indices.push(cell_idx - self.grid.n_rows_and_cols + 1);
            indices.push(cell_idx + 1);
            indices.push(cell_idx + self.grid.n_rows_and_cols + 1);
        }

        indices.push(cell_idx - self.grid.n_rows_and_cols);
        indices.push(cell_idx + self.grid.n_rows_and_cols);

        indices.retain(|&idx| {
            idx < self.grid.cells.len() && !self.grid.obstacle_indices.contains(&idx)
        });
        indices
    }

    fn reconstruct_path(&mut self, cell: Cell) -> Result<(), JsValue> {
        let mut path = vec![];
        let mut cell = Some(cell);

        while cell != None {
            let c = cell.unwrap();
            path.push(c.clone());
            match c.get_parent_cell_idx() {
                Some(idx) => {
                    cell = Some(self.grid.cells.get(idx).expect("Cell must exist").clone());
                }
                None => {
                    cell = None;
                }
            }
        }

        for p in path {
            if p.vec_idx == self.grid.source_cell_idx || p.vec_idx == self.grid.goal_cell_idx {
                continue;
            }
            self.grid
                .cells
                .get(p.vec_idx)
                .expect("Cell must exist")
                .html_element
                .style()
                .set_property("background-color", "yellow")?;
        }

        Ok(())
    }

    async fn search(&mut self) -> Result<(), JsValue> {
        let mut cells = self.grid.cells.clone();

        let mut source_cell = cells
            .get(self.grid.source_cell_idx)
            .expect("Source cell must exist")
            .clone();
        let mut goal_cell = cells
            .get(self.grid.goal_cell_idx)
            .expect("Goal cell must exist")
            .clone();

        let mut open_list = vec![source_cell.clone()];
        let mut closed_list: Vec<Cell> = vec![];

        source_cell.set_g_score(0);
        source_cell.set_h_score(self.euclidean_distance(
            source_cell.col,
            goal_cell.col,
            source_cell.row,
            goal_cell.row,
        ));
        source_cell.set_f_score(source_cell.get_g_score() + source_cell.get_h_score());
        source_cell.set_parent_cell_idx(None);

        while open_list.len() > 0 {
            // let dur = Duration::from_millis(10);
            // Delay::new(dur).await;
            let mut current = self.get_cell_with_lowest_f_score(&open_list);
            if current == goal_cell {
                return self.reconstruct_path(current);
                // return Ok(());
            }
            open_list = open_list
                .iter()
                .filter(|&cell| cell != &current)
                .cloned()
                .collect();
            closed_list.push(current.clone());

            if (current != source_cell && current != goal_cell) {
                // let dur = Duration::from_millis(10);
                // Delay::new(dur).await;
                current
                    .html_element
                    .style()
                    .set_property("background-color", "#87CEEB")?;
            }

            for mut neighbour in self.get_neighbours(current.clone()) {
                if closed_list.contains(&neighbour) {
                    continue;
                }

                let tentative_g = current.get_g_score()
                    + self.euclidean_distance(
                        current.col,
                        neighbour.col,
                        current.row,
                        neighbour.row,
                    );

                if !open_list.contains(&neighbour.clone()) {
                    open_list.push(neighbour.clone());
                } else if tentative_g >= neighbour.get_g_score() {
                    continue;
                }

                neighbour.set_parent_cell_idx(Some(current.clone().vec_idx));
                neighbour.set_g_score(tentative_g);
                neighbour.set_h_score(self.euclidean_distance(
                    neighbour.col,
                    goal_cell.col,
                    neighbour.row,
                    goal_cell.row,
                ));
                neighbour.set_f_score(neighbour.get_g_score() + neighbour.get_h_score());
            }
        }

        Ok(())
    }
}

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    loop {
        let mut grid = Grid::new(20)?;
        let mut a_star = AStar { grid: grid };
        a_star.search().await;
        let dur = Duration::from_millis(3000);
        Delay::new(dur).await;
    }
}
