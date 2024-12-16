from copy import copy

with open("input.txt", "r") as f:
    lines = f.read().splitlines()

def is_report_safe(report):
    increasing = None
    for i, level in enumerate(report[:-1]):
        next = report[i + 1]
        diff = level - next
        if i == 0: increasing = (diff < 0)
        if abs(diff) == 0 or abs(diff) > 3: return False
        else:
            if increasing and diff > 0:  return False
            elif not increasing and diff < 0: return False
    return True
    
def part_1():
    safe_reports = 0
    for line in lines:
        report = list(map(int, line.split()))
        if is_report_safe(report): safe_reports += 1

    print(f"Number of safe reports = {safe_reports}")

def is_report_safe_2(report):
    if is_report_safe(report): return True
    else:
        for i, element in enumerate(report):
            new_report = copy(report)
            new_report.pop(i)
            if is_report_safe(new_report):
                return True
        return False

def part_2():
    safe_reports = 0
    for line in lines:
        report = list(map(int, line.split()))
        if is_report_safe_2(report): safe_reports += 1
    print(f"Number of safe reports if removing one item is allowed = {safe_reports}")

if __name__ == "__main__":
    part_1()
    part_2()