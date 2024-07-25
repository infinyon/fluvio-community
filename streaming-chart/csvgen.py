import math
import csv

with open('sine_wave.csv', 'w', newline='') as csvfile:
    writer = csv.writer(csvfile)
    writer.writerow(['x', 'y'])
    for i in range(500):
        x = i / 10.0
        y = math.sin(x)
        writer.writerow([x, y])
