use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct BingoBoard {
  data: Vec<Vec<BingoSpace>>,
  won: bool
}

impl BingoBoard {
  fn called_number(&mut self, num: &str) {
    for row in &mut self.data {
      for space in row {
        if space.val == num {
          space.mark_called();
        }
      }
    }
  }

  fn sum_uncalled(&self) -> i32 {
    return self.data
      .iter()
      .map(|row|
        row.iter().filter(|space| !space.called).map(|space| space.val.parse::<i32>().unwrap())
        .sum::<i32>()
      )
      .sum()
  }

  fn is_winner(&self) -> bool {
    // return false
    return self.row_winner() || self.column_winner();
  }

  fn row_winner(&self) -> bool {
    return self.data
      .iter()
      .any(|row|
        row.iter().all(|space| space.called )
      );
  }

  fn column_winner(&self) -> bool {
    for i in 0..5 {
      if self.data
          .iter()
          .map(|row| row[i].called)
          .all(|column| column) {
            return true
          }
    }
    return false
  }
}

#[derive(Debug)]
struct BingoSpace {
    val: String,
    called: bool
}

impl BingoSpace {
  fn mark_called(&mut self) {
    self.called = true;
  }
}

fn main() {
    let filename = "src/input.txt";
    part1(filename);
    part2(filename);
}

fn part1(filename: &str) {
    let lines: Vec<String> = std::fs::read_to_string(filename)
        .expect("file not found!")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();
    // println!("{:?}", lines);
    let mut lines_iter = lines.iter();
    let callouts: Vec<&str> = lines_iter.next().unwrap().split(",").collect();
    // println!("callouts {:?}", callouts);

    // skip empty
    lines_iter.next();

    // build boards
    let mut boards: Vec<BingoBoard> = Vec::new();
    let board_lines = &lines[2..];
    // println!("board_lines {:?}", board_lines);
    for chunk in board_lines.chunks(6) {
        // println!("chunk {:?}", chunk);
        // let data: Vec<Vec<BingoSpace>> = Vec::new();
        let data: Vec<Vec<BingoSpace>> = chunk[..5]
          .iter()
          .map(|l|
            l.split(" ")
            .filter(|s| *s != "")
            .map(|u|
              BingoSpace{ val: u.to_string(), called: false }
            )
            .collect()
          )
          .collect();
        let board = BingoBoard {
            data: data,
            won: false
        };
        boards.push(board);
    }
    // println!("boards {:?}", boards)

    // iterate through callouts
    // mark each board with callout
    // check winner
    // if winner, stop
    // else keep going

    for callout in callouts {
      let mut i = 0;
      for board in &mut boards {
        board.called_number(callout);
        if board.is_winner() {
          let sum_uncalled = board.sum_uncalled();
          println!("CALLOUT {} SUM_UNCALLED {}", callout, sum_uncalled);
          println!("CALLOUT {} WINNER BOARD #{}", callout, i);
          println!("result {}", callout.parse::<i32>().unwrap() * sum_uncalled);
          return;
        }
        // board.winner?()
        i += 1;
      }
    }
}

fn part2(filename: &str) {
  let lines: Vec<String> = std::fs::read_to_string(filename)
      .expect("file not found!")
      .lines()
      .map(|x| x.parse().unwrap())
      .collect();
  // println!("{:?}", lines);
  let mut lines_iter = lines.iter();
  let callouts: Vec<&str> = lines_iter.next().unwrap().split(",").collect();
  // println!("callouts {:?}", callouts);

  // skip empty
  lines_iter.next();

  // build boards
  let mut boards: Vec<BingoBoard> = Vec::new();
  let board_lines = &lines[2..];
  // println!("board_lines {:?}", board_lines);
  for chunk in board_lines.chunks(6) {
      // println!("chunk {:?}", chunk);
      // let data: Vec<Vec<BingoSpace>> = Vec::new();
      let data: Vec<Vec<BingoSpace>> = chunk[..5]
        .iter()
        .map(|l|
          l.split(" ")
          .filter(|s| *s != "")
          .map(|u|
            BingoSpace{ val: u.to_string(), called: false }
          )
          .collect()
        )
        .collect();
      let board = BingoBoard {
          data: data,
          won: false
      };
      boards.push(board);
  }
  let num_boards = boards.len();
  let mut winner_count = 0;
  
  // find winner
  // when a board is a winner, check if number of winners match total boards
  // if not, keep going
  // if matches, stop, thats the answer


  for callout in callouts {
    let mut i = 0;
    for board in &mut boards {
      board.called_number(callout);
      if board.is_winner() && !board.won {
        board.won = true;
        winner_count += 1;
        if winner_count == num_boards {
          let sum_uncalled = board.sum_uncalled();
          println!("CALLOUT {} SUM_UNCALLED {}", callout, sum_uncalled);
          println!("CALLOUT {} WINNER BOARD #{}", callout, i);
          println!("result {}", callout.parse::<i32>().unwrap() * sum_uncalled);
          return;
        }
      }
      // board.winner?()
      i += 1;
    }
  }
}