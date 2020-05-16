import MapReduce
import sys

mr = MapReduce.MapReduce()

def mapper(r):
    words = r[1].split()
    for w in words:
        mr.emit_intermediate(w, r[0])


def reducer(key, vals):
    docs = set()
    for v in vals:
        docs.add(v)
    mr.emit((key, list(docs)))


if __name__ == '__main__':
    mr.execute(open(sys.argv[1]), mapper, reducer)
