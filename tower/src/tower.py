import time
import os

class Tower:
    def __init__(self, num_disks):
        self.pegs = [list(range(num_disks, 0, -1)), [], []]
        self.max_disk_size = num_disks
        self.last_moved_disk = None
        self.destination_peg = 2

    def move_disk(self, from_peg, to_peg):
        disk = self.pegs[from_peg].pop()
        self.pegs[to_peg].append(disk)
        self.last_moved_disk = disk

    def display(self):
        os.system('cls' if os.name == 'nt' else 'clear')
        colors = {"red": "\033[31m", "green": "\033[32m", "reset": "\033[0m"}
        
        for row in range(self.max_disk_size, 0, -1):
            line = ""
            for peg_idx in range(3):
                peg = self.pegs[peg_idx]
                if len(peg) >= row:
                    disk = peg[row-1]
                    disk_str = "=" * (disk * 2 - 1)
                    padding = ' ' * (self.max_disk_size - disk)
                    if disk == self.last_moved_disk:
                        disk_str = f"{colors['red']}{disk_str}{colors['reset']}"
                    elif peg_idx == self.destination_peg:
                        disk_str = f"{colors['green']}{disk_str}{colors['reset']}"
                    line += f"{padding}{disk_str}{padding} "
                else:
                    line += " " * (self.max_disk_size - 1) + "|" + " " * (self.max_disk_size - 1) + " "
            print(line)
        print(("=" * (self.max_disk_size * 2) + " ") * 3)
        print("{:^{width}} {:^{width}} {:^{width}}".format("A", "B", "C", width=self.max_disk_size*2))


def tower_of_hanoi(n, source, destination, auxiliary, tower, delay, move_counter):
    if n == 1:
        move_counter[0] += 1
        print(f"Move {move_counter[0]}: Disk 1 from {source} to {destination}")
        tower.move_disk(source, destination)
        tower.display()
        if delay > 0:
            time.sleep(delay)
        return

    tower_of_hanoi(n-1, source, auxiliary, destination, tower, delay, move_counter)

    move_counter[0] += 1
    print(f"Move {move_counter[0]}: Disk {n} from {source} to {destination}")
    tower.move_disk(source, destination)
    tower.display()
    if delay > 0:
        time.sleep(delay)

    tower_of_hanoi(n-1, auxiliary, destination, source, tower, delay, move_counter)


def get_numeric_input(prompt, min_val=1, max_val=20):
    while True:
        try:
            num = int(input(f"{prompt}: "))
            if min_val <= num <= max_val:
                return num
            else:
                print(f"Enter a number between {min_val} and {max_val}.")
        except ValueError:
            print("Invalid input. Enter a numeric value.")


def get_float_input(prompt, min_val=0.000001, max_val=5000):
    while True:
        try:
            num = float(input(f"{prompt}: "))
            if min_val <= num <= max_val:
                return num
            else:
                print(f"Enter a number between {min_val} and {max_val}.")
        except ValueError:
            print("Invalid input. Enter a numeric value.")


def main():
    print("Tower of Hanoi Visualization")
    print("----------------------------")

    num_disks = get_numeric_input("Enter number of disks (1-20)")
    if num_disks > 10:
        total_moves = 2**num_disks - 1
        confirm = input(f"Warning: This requires {total_moves} moves. Continue? (y/n): ")
        if confirm.lower() != 'y':
            print("Exiting.")
            return

    delay_ms = get_float_input("Enter animation delay in milliseconds", 0, 5000)
    delay = delay_ms / 1000

    tower = Tower(num_disks)
    tower.display()
    input("Press Enter to start...")

    move_counter = [0]

    start_time = time.time()
    tower_of_hanoi(num_disks, 0, 2, 1, tower, delay, move_counter)
    elapsed_time = time.time() - start_time

    print(f"\nPuzzle solved in {move_counter[0]} moves!")
    print(f"Total processing time: {elapsed_time:.2f} seconds")
    input("Press Enter to exit...")

if __name__ == '__main__':
    main()