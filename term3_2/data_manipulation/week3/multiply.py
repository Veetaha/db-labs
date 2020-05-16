import MapReduce
import sys

mr = MapReduce.MapReduce()
mr.should_sort = True


def mapper(r):
    matrix = r[0]
    i = r[1]
    j = r[2]
    value = r[3]
    for k in range(5 if matrix == 'a' else 5):
        if matrix == 'a':
            indexes = (i, k)
        else:
            indexes = (k, j)
        mr.emit_intermediate(indexes, r)


def reducer(key, vals):
    n = max(5, 5)
    row = [0] * n
    column = [0] * n
    for x in vals:
        matrix = x[0]
        i = x[1]
        j = x[2]
        value = x[3]
        target = row if matrix == 'a' else column
        target[j if matrix == 'a' else i] = value
    result = 0
    for k in range(n):
        result += row[k] * column[k]
    if result != 0:
        mr.emit(key + (result, ))


if __name__ == '__main__':
    mr.execute(open(sys.argv[1]), mapper, reducer)
