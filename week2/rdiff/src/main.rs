use grid::Grid; // For lcs()
use std::env;
use std::fs::File; // For read_file_lines()
use std::io::{self, BufRead}; // For read_file_lines()
use std::process;

pub mod grid;

/// Reads the file at the supplied path, and returns a vector of strings.
fn read_file_lines(filename: &String) -> Result<Vec<String>, io::Error> {
    let file = File::open(filename)?;
    let mut lines = vec![];
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        lines.push(line);
    }

    Ok(lines)
}

fn lcs(seq1: &Vec<String>, seq2: &Vec<String>) -> Grid {
    // Note: Feel free to use unwrap() in this code, as long as you're basically certain it'll
    // never happen. Conceptually, unwrap() is justified here, because there's not really any error
    // condition you're watching out for (i.e. as long as your code is written correctly, nothing
    // external can go wrong that we would want to handle in higher-level functions). The unwrap()
    // calls act like having asserts in C code, i.e. as guards against programming error.
    let mut common = Grid::new(seq1.len() + 1, seq2.len() + 1);

    seq1.iter().enumerate().for_each(|(i, l)| {
        seq2.iter().enumerate().for_each(|(j, r)| {
            if l == r {
                common
                    .set(i + 1, j + 1, common.get(i, j).unwrap() + 1)
                    .unwrap();
            } else {
                common
                    .set(
                        i + 1,
                        j + 1,
                        std::cmp::max(common.get(i + 1, j).unwrap(), common.get(i, j + 1).unwrap()),
                    )
                    .unwrap();
            }
        })
    });

    common
}

fn print_diff(
    lcs_table: &Grid,
    lines1: &Vec<String>,
    lines2: &Vec<String>,
    i: usize,
    j: usize,
    prefix: usize,
) {
    if i > 0 && j > 0 && lines1[i - 1] == lines2[j - 1] {
        print_diff(&lcs_table, &lines1, &lines2, i - 1, j - 1, prefix);
        println!("{:prefix$}:   {}", i, lines1[i - 1], prefix = prefix);
    } else if j > 0 && (i == 0 || lcs_table.get(i, j - 1) >= lcs_table.get(i - 1, j)) {
        print_diff(&lcs_table, &lines1, &lines2, i, j - 1, prefix);
        println!("{:prefix$}: > {}", j, lines2[j - 1], prefix = prefix);
    } else if i > 0 && (j == 0 || lcs_table.get(i, j - 1) <= lcs_table.get(i - 1, j)) {
        print_diff(&lcs_table, &lines1, &lines2, i - 1, j, prefix);
        println!("{:prefix$}: < {}", i, lines1[i - 1], prefix = prefix);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename1 = &args[1];
    let filename2 = &args[2];

    let file_lines1 =
        read_file_lines(filename1).expect(&format!("failed to read file `{}'", filename1));
    let file_lines2 =
        read_file_lines(filename2).expect(&format!("failed to read file `{}'", filename2));

    print_diff(
        &lcs(&file_lines1, &file_lines2),
        &file_lines1,
        &file_lines2,
        file_lines1.len(),
        file_lines2.len(),
        std::cmp::max(
            (file_lines1.len() as f64).log10() as usize + 1,
            (file_lines2.len() as f64).log10() as usize + 1,
        ),
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_lines() {
        let lines_result = read_file_lines(&String::from("handout-a.txt"));
        assert!(lines_result.is_ok());
        let lines = lines_result.unwrap();
        assert_eq!(lines.len(), 8);
        assert_eq!(
            lines[0],
            "This week's exercises will continue easing you into Rust and will feature some"
        );
    }

    #[test]
    fn test_lcs() {
        let mut expected = Grid::new(5, 4);
        expected.set(1, 1, 1).unwrap();
        expected.set(1, 2, 1).unwrap();
        expected.set(1, 3, 1).unwrap();
        expected.set(2, 1, 1).unwrap();
        expected.set(2, 2, 1).unwrap();
        expected.set(2, 3, 2).unwrap();
        expected.set(3, 1, 1).unwrap();
        expected.set(3, 2, 1).unwrap();
        expected.set(3, 3, 2).unwrap();
        expected.set(4, 1, 1).unwrap();
        expected.set(4, 2, 2).unwrap();
        expected.set(4, 3, 2).unwrap();

        println!("Expected:");
        expected.display();
        let result = lcs(
            &"abcd".chars().map(|c| c.to_string()).collect(),
            &"adb".chars().map(|c| c.to_string()).collect(),
        );
        println!("Got:");
        result.display();
        assert_eq!(result.size(), expected.size());
        for row in 0..expected.size().0 {
            for col in 0..expected.size().1 {
                assert_eq!(result.get(row, col), expected.get(row, col));
            }
        }
    }
}
