#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Delimiter {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug)]
enum Bracket {
    Opening(Delimiter),
    Closing(Delimiter),
}

impl Delimiter {
    pub fn from_char(c: char) -> Result<Self, char> {
        match c {
            '(' | ')' => Ok(Delimiter::Paren),
            '[' | ']' => Ok(Delimiter::Bracket),
            '{' | '}' => Ok(Delimiter::Brace),
            '<' | '>' => Ok(Delimiter::Angle),
            c => Err(c),
        }
    }
    
    pub fn chars(&self) -> (char, char) {
        match self {
            Delimiter::Paren   => ('(', ')'),
            Delimiter::Bracket => ('[', ']'),
            Delimiter::Brace   => ('{', '}'),
            Delimiter::Angle   => ('<', '>'),
        }
    }

    pub fn error_score(&self) -> usize {
        match self {
            Delimiter::Paren => 3,
            Delimiter::Bracket => 57,
            Delimiter::Brace => 1197,
            Delimiter::Angle => 25137,
        }
    }

    pub fn completion_score(&self) -> usize {
        match self {
            Delimiter::Paren => 1,
            Delimiter::Bracket => 2,
            Delimiter::Brace => 3,
            Delimiter::Angle => 4,
        }
    }
}

impl Bracket {
    pub fn from_char(c: char) -> Result<Self, char> {
        let delim = Delimiter::from_char(c)?;
        if c == delim.chars().0 {
            Ok(Bracket::Opening(delim))
        } else {
            Ok(Bracket::Closing(delim))
        }
    }
}

// Return:
//   Ok: list of delimiters needed to complete a line (in order inner -> outer)
//   Err: (column, Delimiter), when a wrong delimiter has been found
fn complete_line(line: &str) -> Result<Vec<Delimiter>, (usize, Delimiter)> {
    let mut stack = Vec::new();
    for (i, c) in line.chars().enumerate() {
        let bracket = Bracket::from_char(c).unwrap();
        match bracket {
            Bracket::Opening(d) => stack.push(d),
            Bracket::Closing(d) => {
                let expected = stack.pop()
                    .expect("Found closing delimiter without an opening one");
                if d != expected {
                    return Err((i, d));
                }
            },
        }
    }
    Ok(stack.iter().cloned().rev().collect())
}

pub fn part_1(lines: &[String], verbose: bool) -> usize {
    lines.iter()
        .map(|line| complete_line(&line))
        .enumerate()
        .filter_map(|(line, result)| result.err()
                    .map(|(col, delim)| (line, col, delim)))
        .inspect(|(line, col, delim)| {
            if verbose {
                println!("Error on line {}, at column {}, delimiter = {}", 
                     line, col, delim.chars().1);
            }
        })
        .map(|(_, _, delim)| delim.error_score())
        .sum()
}

fn completion_score(delims: &[Delimiter]) -> usize {
    delims.iter()
        .fold(0, |score, delim| score * 5 + delim.completion_score())
}

pub fn part_2(lines: &[String], verbose: bool) -> usize {
    let mut scores: Vec<_> = lines.iter()
        .map(|line| complete_line(&line))
        .enumerate()
        .filter_map(|(line, result)| result.ok()
                    .map(|delims| {
                        let score = completion_score(&delims);
                        (line, delims, score)
                    }))
        .inspect(|(line, delims, score)| {
            if verbose {
                let delims: Vec<_> = delims.iter()
                    .map(|d| d.chars().1.to_string())
                    .collect();
                println!("Completion for line {}: {} for {} points", 
                     line, delims.join(""), score);
            }
        })
        .map(|(_, _, score)| score)
        .collect();
    scores.sort_unstable();
    assert!(scores.len() % 2 != 0, "Scores count is even");
    scores[scores.len() / 2]
}
