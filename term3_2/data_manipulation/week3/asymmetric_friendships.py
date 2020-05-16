import MapReduce
import sys

mr = MapReduce.MapReduce()


if __name__ == '__main__':
    mr.execute(open(sys.argv[1]), mapper, reducer)

def mapper(record):
    p = record[0]
    f = record[1]
    mr.emit_intermediate(p, f)
    mr.emit_intermediate(f, p)


def reducer(key, vals):
    fs = set()
    for x in vals:
        if x in fs:
            fs.remove(x)
        else:
            fs.add(x)
    for x in fs:
        mr.emit((key, x))
