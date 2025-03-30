
function delay(ms) {
  return new Promise(resolve => setTimeout(resolve, ms));
}

class Grid {
  constructor(nRowsAndCols) {
    this.nRowsAndCols = nRowsAndCols;
    this.htmlCellContainer = null;
    this.setup();
  }

  makeCells() {

    const cells = []
    for (let row = 0; row < this.nRowsAndCols; row++) {
      for (let col = 0; col < this.nRowsAndCols; col++) {

        const cell = document.createElement('div');
        cell.style.width = '20px';
        cell.style.height = '20px';
        cell.style.backgroundColor = 'green';
        cell.style.border = '1px solid black';
        cell.row = row;
        cell.col = col;
        cell.arrIdx = (row * this.nRowsAndCols) + col;
        cell.addEventListener('click', () => { console.log(this.getNeighbours(cell)) })
        cells.push(cell);
      }
    }
    return cells;
  }

  randomIntArray(n, z) {
    // console.log(randomIntArray(5, 10)); // [3, 7, 1, 9, 4] 
    return Array.from({ length: n }, () => Math.floor(Math.random() * (z + 1)));
  }


  addRandomObstacles() {

    // 50% of the cells will be obstacles
    const nObstacles = Math.floor(0.5 * (this.nRowsAndCols * this.nRowsAndCols - 1))
    let obstacleIndices = this.randomIntArray(nObstacles, this.nRowsAndCols * this.nRowsAndCols - 1)
    for (const obstacleIndex of obstacleIndices) {
      let cellCandidate = this.cells[obstacleIndex]
      if (cellCandidate !== this.sourceCell && cellCandidate !== this.goalCell) {
        cellCandidate.isObstacle = true
      }
    }
  }

  getNeighbours(cell) {
    const neighbours = []
    const arr_idx = cell.arrIdx
    const indices = []

    if (cell.col !== 0) {
      // up left 
      indices.push(arr_idx - this.nRowsAndCols - 1)
      // left
      indices.push(arr_idx - 1)
      // down left
      indices.push(arr_idx + this.nRowsAndCols - 1)
    }

    if (cell.col !== this.nRowsAndCols - 1) {
      // up right
      indices.push(arr_idx - this.nRowsAndCols + 1)
      // right
      indices.push(arr_idx + 1)
      // down right
      indices.push(arr_idx + this.nRowsAndCols + 1)
    }


    // up
    indices.push(arr_idx - this.nRowsAndCols)
    // down 
    indices.push(arr_idx + this.nRowsAndCols)

    for (let index of indices) {
      const neighbour = this.safeGetCellFromCells(index)
      if (neighbour) {
        neighbours.push(neighbour)
      }
    }

    return neighbours
  }

  safeGetCellFromCells(index) {
    try {
      const cell = this.cells[index]
      if (!cell.isObstacle) {
        return cell
      }
      return null;
    } catch (error) {
      return null;
    }
  }

  setup() {
    this.cells = this.makeCells();
    // this.sourceCell = this.cells[0];
    // this.goalCell = this.cells[this.cells.length - 1];
    this.sourceCell = this.getRandomSourceCell();
    this.goalCell = this.getRandomGoalCell();
    this.addRandomObstacles();
    this.draw()
  }


  getRandomCell() {
    return this.cells[Math.floor(Math.random() * this.cells.length)]
  }

  getRandomSourceCell() {
    return this.getRandomCell()
  }

  getRandomGoalCell() {
    let selection = this.getRandomCell()
    while (selection === this.sourceCell) {
      selection = this.getRandomCell()
    }
    return selection;
  }

  reset() {
    this.setup();
  }

  highlightCell(index, colour) {
    this.cells[index].style.backgroundColor = colour;
  }


  draw() {
    if (this.htmlCellContainer) {
      document.body.removeChild(this.htmlCellContainer);
    }


    document.body.style.display = 'flex';
    document.body.style.justifyContent = 'center';
    document.body.style.alignItems = 'center';
    document.body.style.height = '100vh';
    document.body.style.margin = '0';

    this.htmlCellContainer = document.createElement('div');
    this.htmlCellContainer.style.display = 'flex';
    this.htmlCellContainer.style.flexDirection = 'column';

    const cellSize = `calc(min(90vw, 90vh) / ${this.nRowsAndCols})`;


    for (let r = 0; r < this.nRowsAndCols; r++) {
      const row = document.createElement('div');
      row.style.display = 'flex';
      for (let c = 0; c < this.nRowsAndCols; c++) {
        const cell = this.cells[(r * this.nRowsAndCols) + c];
        if (cell === this.sourceCell) {
          cell.style.backgroundColor = 'blue';
        } else if (cell === this.goalCell) {
          cell.style.backgroundColor = 'red';
        } else if (cell.isObstacle) {
          cell.style.backgroundColor = '#654321'
        } else {
          cell.style.backgroundColor = 'green';
        }
        cell.style.width = cellSize;
        cell.style.height = cellSize;
        row.appendChild(cell);
      }
      this.htmlCellContainer.appendChild(row);
    }
    document.body.appendChild(this.htmlCellContainer);
  }
}

class AStarAlgorithm {

  constructor(grid) {
    this.grid = grid;
  }

  euclideanDistance(x1, x2, y1, y2) {
    return Math.sqrt(((x1 - x2) ** 2) + ((y1 - y2) ** 2))
  }

  getCellWithLowestFScore(openList) {
    return openList.reduce((acc, cell) => {
      return cell.f < acc.f ? cell : acc;
    }, openList[0])
  }


  reconstructPath(current) {
    const path = []
    while (current != null) {
      path.push(current)
      current = current.parent
    }

    for (let cell of path) {
      if (cell != this.grid.sourceCell && cell !== this.grid.goalCell) {
        cell.style.backgroundColor = 'yellow'
      }
    }
    return path
  }



  async run() {
    const sourceCell = this.grid.sourceCell;
    const goalCell = this.grid.goalCell;

    const openList = [sourceCell];
    const closedList = [];

    sourceCell.g = 0;
    sourceCell.h = this.euclideanDistance(sourceCell.col, goalCell.col, sourceCell.row, goalCell.row);
    sourceCell.f = sourceCell.g + sourceCell.h;
    sourceCell.parent = null;
    while (openList.length > 0) {

      await delay(50);
      const current = this.getCellWithLowestFScore(openList);
      if (current === goalCell) {
        return this.reconstructPath(current);
      }
      openList.splice(openList.indexOf(current), 1)
      closedList.push(current)

      if (current !== sourceCell && current !== goalCell) {
        this.grid.highlightCell(current.arrIdx, '#AAF8E6')
      }

      for (let neighbour of this.grid.getNeighbours(current)) {
        if (closedList.includes(neighbour)) {
          continue
        }

        const tentative_g = current.g + this.euclideanDistance(current.col, neighbour.col, current.row, neighbour.row)

        if (!openList.includes(neighbour)) {
          openList.push(neighbour)
        } else if (tentative_g >= neighbour.g) {
          continue
        }

        neighbour.parent = current
        neighbour.g = tentative_g
        neighbour.h = this.euclideanDistance(neighbour.col, goalCell.col, neighbour.row, goalCell.row);
        neighbour.f = neighbour.g + neighbour.h
      }
    }

  }
}

class GameManager {

  constructor(nRowsAndCols) {
    this.grid = new Grid(nRowsAndCols);
    this.aStarAlgorithm = new AStarAlgorithm(this.grid);
  }



  async loop() {
    while (true) {
      await this.aStarAlgorithm.run();
      await delay(1000);
      this.grid.reset();
    }
  }

}


const gameManager = new GameManager(20);
gameManager.loop();
