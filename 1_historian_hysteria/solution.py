with open("input.txt", "r") as f: data = f.read().splitlines()

lefts = []
rights = []
for item in data:
    left, right = item.split()
    lefts.append(int(left))
    rights.append(int(right))

def part_1():    
    total_distance = 0
    for left, right in zip(sorted(lefts), sorted(rights)):
        total_distance += abs(left - right)
        # print(f"({left}, {right}) => {abs(left - right)}")
    print(f"Total distance = {total_distance}")

def part_2():
    score = 0
    for left in lefts:
        repeats = 0
        for right in rights:
            if right == left: repeats += 1
        score += left*repeats
    print(f"Similarity score = {score}")

if __name__ == "__main__":
    part_1()
    part_2()