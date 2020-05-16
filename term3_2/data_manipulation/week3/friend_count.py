import MapReduce
import sys

mr = MapReduce.MapReduce()

def mapper(record):
    mr.emit_intermediate(record[0], 1)

def reducer(key, vals):
    mr.emit((key, len(vals)))

if __name__ == '__main__':
    mr.execute(open(sys.argv[1]), mapper, reducer)
