import MapReduce
import sys

mr = MapReduce.MapReduce()

def mapper(r):
    res = r[1][:-10]
    mr.emit_intermediate(len(res), res)


def reducer(_, vals):
    acc = set()
    for x in vals:
        acc.add(x)
    for x in acc:
        mr.emit(x)


if __name__ == '__main__':
    mr.execute(open(sys.argv[1]), mapper, reducer)
