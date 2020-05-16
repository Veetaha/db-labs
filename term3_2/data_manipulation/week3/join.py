import MapReduce
import sys

mr = MapReduce.MapReduce()

def mapper(r):
    mr.emit_intermediate(r[1], {
        'table': r[0],
        'payload': r
    })


def reducer(_key, vals):
    orders = []
    lines = []
    for x in vals:
        dest = orders if x['table'] == 'order' else lines
        dest.append(x['payload'])
    for order in orders:
        for line in lines:
            mr.emit((order + line))

if __name__ == '__main__':
    inputdata = open(sys.argv[1])
    mr.execute(inputdata, mapper, reducer)
