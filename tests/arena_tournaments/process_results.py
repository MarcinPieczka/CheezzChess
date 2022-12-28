#!python3
import argparse
import re
from pprint import pprint

parser = argparse.ArgumentParser(
    description='Script to process Arena Chess Turnament files')
parser.add_argument('file_path', help='Path to png file')
args = parser.parse_args()



pattern = re.compile(r'\[(.*) \"(.*)\"\]')

results = []
current = None

with open(args.file_path) as f:
    for line in f.readlines():
        match = re.match(pattern, line)
        if match:
            current = current or {}
            current[match.group(1)] = match.group(2)
        elif current:
            results.append(current)
            
avg_game_length = sum(int(res["PlyCount"]) for res in results) / (len(results) * 2)
print(f"Average game length: {avg_game_length}")
