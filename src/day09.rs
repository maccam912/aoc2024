use crate::Solution;

#[derive(Debug, Clone)]
struct File {
    blocks: Vec<usize>, // Positions of blocks belonging to this file
}

#[derive(Debug)]
struct DiskMap {
    files: Vec<File>,
    disk: Vec<Option<usize>>, // None for free space, Some(id) for file blocks
}

impl DiskMap {
    fn from_string(input: &str) -> Self {
        let mut current_pos = 0;
        let mut file_id = 0;
        let mut files = Vec::new();
        let mut disk = Vec::new();

        let numbers: Vec<usize> = input
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        for (i, &length) in numbers.iter().enumerate() {
            if i % 2 == 0 {
                // File
                let mut blocks = Vec::new();
                for pos in current_pos..current_pos + length {
                    blocks.push(pos);
                    disk.push(Some(file_id));
                }
                files.push(File { blocks });
                file_id += 1;
            } else {
                // Free space
                for _ in 0..length {
                    disk.push(None);
                }
            }
            current_pos += length;
        }

        DiskMap { files, disk }
    }

    fn move_block_left(&mut self) -> bool {
        // Find the leftmost free space
        if let Some(free_pos) = self.disk.iter().position(|x| x.is_none()) {
            // Find the rightmost file block
            if let Some(block_pos) = self.disk.iter().rposition(|x| x.is_some()) {
                if block_pos > free_pos {
                    // Get the file ID of the block we're moving
                    let file_id = self.disk[block_pos].unwrap();

                    // Update the disk
                    self.disk[free_pos] = Some(file_id);
                    self.disk[block_pos] = None;

                    // Update the file's block positions
                    let file = &mut self.files[file_id];
                    file.blocks.retain(|&x| x != block_pos);
                    file.blocks.push(free_pos);
                    file.blocks.sort_unstable();

                    return true;
                }
            }
        }
        false
    }

    fn move_file_left(&mut self, file_id: usize) -> bool {
        let file_size = self.files[file_id].blocks.len();
        let file_start = self.files[file_id].blocks[0];

        // Find leftmost span of free space that can fit the file
        let mut current_span = 0;
        let mut span_start = None;

        for (pos, &block) in self.disk.iter().enumerate() {
            if pos >= file_start {
                break;
            }

            if block.is_none() {
                if current_span == 0 {
                    span_start = Some(pos);
                }
                current_span += 1;
                if current_span >= file_size {
                    // Found a suitable span
                    let target_start = span_start.unwrap();

                    // Move the file
                    for (i, &old_pos) in self.files[file_id].blocks.iter().enumerate() {
                        self.disk[old_pos] = None;
                        self.disk[target_start + i] = Some(file_id);
                    }

                    // Update file's block positions
                    self.files[file_id].blocks = (target_start..target_start + file_size).collect();

                    return true;
                }
            } else {
                current_span = 0;
                span_start = None;
            }
        }
        false
    }

    fn calculate_checksum(&self) -> usize {
        let mut checksum = 0;
        for (pos, &block) in self.disk.iter().enumerate() {
            if let Some(file_id) = block {
                checksum += pos * file_id;
            }
        }
        checksum
    }
}

pub struct Day09;

impl Solution for Day09 {
    fn part1(&self, input: &str) -> String {
        let mut disk_map = DiskMap::from_string(input.trim());

        // Keep moving blocks until no more moves are possible
        while disk_map.move_block_left() {}

        disk_map.calculate_checksum().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut disk_map = DiskMap::from_string(input.trim());

        // Move files from highest ID to lowest
        for file_id in (0..disk_map.files.len()).rev() {
            disk_map.move_file_left(file_id);
        }

        disk_map.calculate_checksum().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_sample() {
        let input = "2333133121414131402";
        assert_eq!(Day09.part1(input), "1928");
    }

    #[test]
    fn test_part2_sample() {
        let input = "2333133121414131402";
        assert_eq!(Day09.part2(input), "2858");
    }
}
