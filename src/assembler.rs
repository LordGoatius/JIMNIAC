use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Debug)]
pub struct Assembler {
    file: BufReader<File>,
    line: usize,
}

impl Assembler {
    pub fn new(input_name: &Path) -> Result<Self, io::Error>{
        Ok(Assembler {
            file: BufReader::new(File::open(input_name)?),
            line: 0,
        })
    }

    pub fn assemble(&mut self, output_name: &Path) -> Result<(), std::io::Error> {
        let outfile = File::create(output_name)?;
        let mut buf_writer = BufWriter::new(outfile);

        let mut line_buf = String::new();
        let mut counter = 0usize;
        let mut label_found = false;
        while let Ok(num) = self.file.read_line(&mut line_buf) {
            if num == 0 { break; }
            // For any extra newlines, this should just keep going
            // Technically is a buggy implementation, but idc, this is my test run
            if num == 1 { continue; }

            // TODO: when process P instr, turn number into btern
            // NOTE: Only accepts btern :), and command only accepts labels

            let mut label_map: HashMap<&str, usize> = HashMap::new();
            
            let mut line = line_buf.trim().split_whitespace();

            match line.next().expect("Empty line") {
                "lsh"  | "LSH"  => buf_writer.write(&[b'0', b'0', b'0'])?,
                "rsh"  | "RSH"  => buf_writer.write(&[b'0', b'0', b'1'])?,
                "neg"  | "NEG"  => buf_writer.write(&[b'0', b'0', b'2'])?,
                "and"  | "AND"  => buf_writer.write(&[b'0', b'1', b'0'])?,
                "or"   | "OR"   => buf_writer.write(&[b'0', b'1', b'1'])?,
                "xor"  | "XOR"  => buf_writer.write(&[b'0', b'1', b'2'])?,
                "add"  | "ADD"  => buf_writer.write(&[b'0', b'2', b'0'])?,
                "sub"  | "SUB"  => buf_writer.write(&[b'0', b'2', b'1'])?,
                "mul"  | "MUL"  => buf_writer.write(&[b'0', b'2', b'2'])?,
                "add3" | "ADD3" => buf_writer.write(&[b'1', b'2', b'0'])?,
                "sub3" | "SUB3" => buf_writer.write(&[b'1', b'2', b'1'])?,
                "mul3" | "MUL3" => buf_writer.write(&[b'1', b'2', b'2'])?,
                "add9" | "ADD9" => buf_writer.write(&[b'2', b'2', b'0'])?,
                "sub9" | "SUB9" => buf_writer.write(&[b'2', b'2', b'1'])?,
                "mul9" | "MUL9" => buf_writer.write(&[b'2', b'2', b'2'])?,
                "cmp"  | "CMP"  => buf_writer.write(&[b'1', b'0', b'0'])?,
                "br"   | "BR"   => buf_writer.write(&[b'1', b'0', b'1'])?,
                "bne"  | "BNE"  => buf_writer.write(&[b'1', b'0', b'2'])?,
                "bgt"  | "BGT"  => buf_writer.write(&[b'1', b'1', b'0'])?,
                "blt"  | "BLT"  => buf_writer.write(&[b'1', b'1', b'1'])?,
                "beq"  | "BEQ"  => buf_writer.write(&[b'1', b'1', b'2'])?,
                "pt"   | "PT"   => { 
                    let _ = buf_writer.write(&[b'2', b'0', b'0'])?; 
                    buf_writer.write(Assembler::parse_num(1, line.next().expect("Provide an immediate to load")))?
                },
                "pth"  | "PTH"  => { 
                    let _ = buf_writer.write(&[b'2', b'0', b'1'])?; 
                    buf_writer.write(Assembler::parse_num(3, line.next().expect("Provide an immediate to load")))?
                },
                "pw"   | "PW"   => { 
                    let _ = buf_writer.write(&[b'2', b'0', b'2'])?; 
                    buf_writer.write(Assembler::parse_num(9, line.next().expect("Provide an immediate to load")))?
                },
                "pct"  | "PCT"  => { 
                    let _ = buf_writer.write(&[b'2', b'1', b'0'])?; 
                    // TODO: make it go to label
                    buf_writer.write(&Assembler::parse_label::<3>(*label_map.get(line.next().expect("Provide a label to load")).expect("Labels must be defined before using them")))?
                },
                "pcth" | "PCTH" => { 
                    let _ = buf_writer.write(&[b'2', b'1', b'1'])?; 
                    // TODO: make it go to label
                    buf_writer.write(&Assembler::parse_label::<9>(*label_map.get(line.next().expect("Provide a label to load")).expect("Labels must be defined before using them")))?
                },
                "pcw"  | "PCW"  => { 
                    let _ = buf_writer.write(&[b'2', b'1', b'2'])?; 
                    // TODO: make it go to label
                    buf_writer.write(&Assembler::parse_label::<27>(*label_map.get(line.next().expect("Provide a label to load")).expect("Labels must be defined before using them")))?
                },
                label => {
                    if label.chars().last().unwrap() != ':' {
                        panic!("Incorrect label syntax");
                    }
                    if let Err(err) = label_map.try_insert(label, counter) {
                        panic!("Label name {} already used  {}", err.entry.key(), err.entry.get());
                    }
                    label_found = true;
                    label.len()
                }
            };

            line_buf.clear();
            if label_found {
                label_found = false;
                continue;
            }
            counter += 1;
        }
        Ok(())
    }

    fn parse_num(_len: usize, input: &str) -> &[u8] {
        match &input[..3] {
            "0t" => input[3..].as_bytes(),
            _ => todo!(),
        }
    }

    fn parse_label<const S: usize>(mut input: usize) -> [u8; S] {
        let mut result = String::new();
        let mut nums = [b'0'; S];
        while input > 0 {
            let mut rem: isize = (input % 3) as isize;
            input /= 3;
            if rem == 2 {
                rem = -1;
                input += 1;
            }
            let append = if rem == 0 { '0' } else if rem == 1 { '1' } else { 'Z' };
            result = format!("{append}{result}");
        }
        println!("{result}");
        for (i, char) in result.chars().rev().enumerate() {
            match char {
                '0' => nums[i] = b'0',
                '1' => nums[i] = b'1',
                'Z' => nums[i] = b'2',
                _   => panic!("Should be impossible"),
            }
        }
        nums
    }
}

#[cfg(test)]
mod test {
    use crate::assembler::Assembler;

    #[test]
    fn usize_to_bal() {
        for i in 0usize..=13 {
            unsafe {
                println!("{i}: {:?}", String::from_utf8_unchecked(Assembler::parse_label::<3>(i).to_vec()));
            }
        }
        for i in 26..=81 {
            unsafe {
                println!("{i}: {:?}", String::from_utf8_unchecked(Assembler::parse_label::<9>(i).to_vec()));
            }
        }
    }
}
