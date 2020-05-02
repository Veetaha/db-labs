import sys
import json

if __name__ == '__main__':
    afinn_file = open(sys.argv[1])
    tweet_file = open(sys.argv[2])
    scores = {}
    for line in afinn_file:
        term, score = line.split("\t")
        scores[term] = int(score)
    afinn_file.seek(0)

    terms = {}
    for line in tweet_file:
        current_score = 0
        json_data = json.loads(line)
        text = json_data['text'] if 'text' in json_data else ""
        words = text.split()
        for word in words:
            current_score += scores[word] if word in scores else 0
        for word in words:
            if word not in scores:
                terms[word] = current_score
    tweet_file.seek(0)

    for term in terms.items():
        print term[0], term[1]
