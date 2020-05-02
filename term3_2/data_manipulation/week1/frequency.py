import sys
import json

if __name__ == '__main__':
    tweet_file = open(sys.argv[1])
    terms = {}
    for line in tweet_file:
        json_data = json.loads(line)
        text = json_data['text'] if 'text' in json_data else ""
        words = text.split()
        for word in words:
            terms[word] = 1 + (terms[word] if word in terms else 0)
    tweet_file.seek(0)
    count_terms = float(len(terms))
    for term in terms.items():
        print term[0], term[1] / count_terms
