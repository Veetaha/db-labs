create index news_body_tsvector_idx on news using gin(to_tsvector('english', body));
