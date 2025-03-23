use std::io::{self, Write, stdin, stdout};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct Tower {
    pegs: Vec<Vec<u32>>,
    max_disk_size: u32,
    last_moved_disk: Option<u32>, // Track the last moved disk
    destination_peg: usize, // Track the destination peg
}

impl Tower {
    fn new(num_disks: u32) -> Self {
        let mut pegs = vec![Vec::new(), Vec::new(), Vec::new()];
        // Initialize the first peg with disks in decreasing order
        for disk in (1..=num_disks).rev() {
            pegs[0].push(disk);
        }
        
        Tower {
            pegs,
            max_disk_size: num_disks,
            last_moved_disk: None,
            destination_peg: 2, // Default destination is peg C (index 2)
        }
    }
    
    fn move_disk(&mut self, from: usize, to: usize) {
        if let Some(disk) = self.pegs[from].pop() {
            self.last_moved_disk = Some(disk); // Set the last moved disk
            self.pegs[to].push(disk);
        }
    }
    
    fn display(&self) {
        // ANSI color codes
        let red = "\x1B[31m";
        let green = "\x1B[32m";
        let reset = "\x1B[0m";
        
        // Instead of clearing the entire screen, just move cursor up to overwrite previous display
        // We'll use a single buffer for the output
        let mut output = String::new();
        
        // Get the height of the tallest peg
        let max_height = self.max_disk_size as usize;
        
        // For each row (starting from the top)
        for row in 0..max_height {
            // For each peg
            for peg_idx in 0..3 {
                let peg = &self.pegs[peg_idx];
                // Calculate the correct row from the top
                let position_from_top = max_height - row - 1;
                
                // If there's a disk at this position in the peg
                if position_from_top < peg.len() {
                    let disk_size = peg[position_from_top];
                    let disk_width = disk_size * 2 - 1;
                    let padding = self.max_disk_size - disk_size;
                    
                    // Add the disk to output
                    output.push_str(&" ".repeat(padding as usize));
                    
                    // Color the disk: red if it's the last moved disk, green if it's on the destination peg
                    if self.last_moved_disk == Some(disk_size) {
                        output.push_str(red);
                        output.push_str(&"=".repeat(disk_width as usize));
                        output.push_str(reset);
                    } else if peg_idx == self.destination_peg {
                        output.push_str(green);
                        output.push_str(&"=".repeat(disk_width as usize));
                        output.push_str(reset);
                    } else {
                        output.push_str(&"=".repeat(disk_width as usize));
                    }
                    
                    output.push_str(&" ".repeat(padding as usize));
                } else {
                    // Add just the peg rod to output
                    output.push_str(&" ".repeat((self.max_disk_size - 1) as usize));
                    output.push('|');
                    output.push_str(&" ".repeat((self.max_disk_size - 1) as usize));
                }
                
                // Less space between pegs
                output.push(' ');
            }
            output.push('\n');
        }
        
        // Add the base
        for _ in 0..3 {
            output.push_str(&"=".repeat((self.max_disk_size * 2) as usize));
            output.push(' ');
        }
        output.push('\n');
        
        // Add peg labels
        let labels = ["A", "B", "C"];
        for i in 0..3 {
            let padding = self.max_disk_size as usize - 1;
            output.push_str(&" ".repeat(padding));
            output.push_str(labels[i]);
            output.push_str(&" ".repeat(padding));
            output.push(' ');
        }
        output.push('\n');
        
        // Clear the screen and move cursor to the top
        print!("\x1B[2J\x1B[1;1H");
        // Print the consolidated output
        print!("{}", output);
        io::stdout().flush().unwrap();
    }
}

fn tower_of_hanoi_visual(n: u32, source: usize, destination: usize, auxiliary: usize, 
                         tower: &mut Tower, delay_ms: u64, move_count: &mut u32) {
    if n == 1 {
        // Move the smallest disk
        *move_count += 1;
        print!("\rMove {}: Disk 1 from {} to {} ", 
               move_count, ['A', 'B', 'C'][source], ['A', 'B', 'C'][destination]);
        io::stdout().flush().unwrap();
        tower.move_disk(source, destination);
        tower.display();
        thread::sleep(Duration::from_millis(delay_ms));
        return;
    }
    
    // Move n-1 disks from source to auxiliary
    tower_of_hanoi_visual(n-1, source, auxiliary, destination, tower, delay_ms, move_count);
    
    // Move the nth disk from source to destination
    *move_count += 1;
    print!("\rMove {}: Disk {} from {} to {} ", 
           move_count, n, ['A', 'B', 'C'][source], ['A', 'B', 'C'][destination]);
    io::stdout().flush().unwrap();
    tower.move_disk(source, destination);
    tower.display();
    thread::sleep(Duration::from_millis(delay_ms));
    
    // Move n-1 disks from auxiliary to destination
    tower_of_hanoi_visual(n-1, auxiliary, destination, source, tower, delay_ms, move_count);
}

fn solve_tower_of_hanoi_visual(n: u32, delay_ms: u64) {
    let mut tower = Tower::new(n);
    println!("Initial state:");
    tower.display();
    thread::sleep(Duration::from_millis(delay_ms));
    
    let mut move_count = 0;
    let start_time = Instant::now(); // Start timing
    tower_of_hanoi_visual(n, 0, 2, 1, &mut tower, delay_ms, &mut move_count);
    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    
    let total_moves = (1 << n) - 1; // 2^n - 1
    println!("\nPuzzle solved in {} moves!", total_moves);
    println!("Total processing time: {:.2?}", elapsed_time); // Print elapsed time
}

// Helper function to get numeric input from terminal
fn get_numeric_input(prompt: &str) -> u32 {
    loop {
        print!("{}: ", prompt);
        stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number.")
        }
    }
}

fn main() {
    println!("Tower of Hanoi Visualization");
    println!("----------------------------");
    
    // Get number of disks (with input validation)
    let num_disks = get_numeric_input("Enter number of disks (1-20)");
    if num_disks > 20 {
        println!("Maximum number of disks is 20. Setting to 20.");
        let num_disks = 20;
    }
    
    // Warning about large numbers of disks
    if num_disks > 10 {
        println!("Warning: Large numbers of disks will take a long time to solve.");
        println!("The solution requires 2^n - 1 moves ({}), continue? (y/n)", (1 << num_disks) - 1);
        
        let mut response = String::new();
        stdin().read_line(&mut response).expect("Failed to read input");
        if !response.trim().eq_ignore_ascii_case("y") {
            println!("Exiting program.");
            return;
        }
    }
    
    // Get animation delay
    let delay_ms = get_numeric_input("Enter animation delay in milliseconds (e.g., 300)");
    
    println!("\nSolving Tower of Hanoi with {} disks...", num_disks);
    println!("Press Enter to start...");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer).expect("Failed to read input");
    
    // Convert delay_ms from u32 to u64 using into()
    solve_tower_of_hanoi_visual(num_disks, delay_ms.into());
    
    println!("Press Enter to exit...");
    buffer.clear();
    stdin().read_line(&mut buffer).expect("Failed to read input");
}