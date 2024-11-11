# rand-sql

generate sql by running
```bash
$ cargo run -- 100
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target\debug\rand-sql.exe 100`
SELECT category, price, created_at FROM customers WHERE quantity = 57 AND id >= 51 AND updated_at BETWEEN TO_DATE('2022-04-18', 'YYYY-MM-DD') AND TO_DATE('2021-05-02', 'YYYY-MM-DD')
SELECT category, id, updated_at FROM products WHERE created_at BETWEEN TO_DATE('2021-03-26', 'YYYY-MM-DD') AND TO_DATE('2022-07-15', 'YYYY-MM-DD') AND price > 65 AND category IN ('ktacq', 'ipyso', 'cudex', 'ijzcg', 'wfktw')
INSERT INTO users (status, id, updated_at) VALUES ('ngrao', 77, TO_DATE('2022-02-28', 'YYYY-MM-DD'))
SELECT status, price, updated_at FROM products WHERE category IN ('fjyec', 'iviph') AND name IN ('hulvt', 'uxcnv', 'ycocw', 'bnkof', 'cduoh', 'pzbmy', 'xsmcv') AND category IN ('dagkb', 'rlgun', 'mfvxo', 'nnnms', 'kpjre', 'phudm', 'fztwm', 'iojcl', 'xfovm', 'exjqx') AND created_at BETWEEN TO_DATE('2022-08-22', 'YYYY-MM-DD') AND TO_DATE('2020-12-20', 'YYYY-MM-DD')
SELECT category, price, created_at FROM orders WHERE category IN ('gvglt', 'nzybs', 'yngkh', 'seqwq', 'pzryl', 'gsxgz', 'mdyzm', 'qnsqo') AND quantity < 53 AND id > 13
SELECT name, price, updated_at FROM orders WHERE quantity > 80 AND updated_at BETWEEN TO_DATE('2021-07-17', 'YYYY-MM-DD') AND TO_DATE('2020-05-01', 'YYYY-MM-DD') AND status IN ('gdyan', 'qdcxz', 'alqtp', 'ylvbb', 'rqtmi', 'iujyx', 'cpyan')
UPDATE customers SET quantity = 27 WHERE status IN ('yhpmc', 'dmxmy', 'lvfze', 'joxse', 'gqyye', 'bqckj') AND price > 27 AND id >= 78 AND quantity > 24 AND created_at BETWEEN TO_DATE('2020-05-28', 'YYYY-MM-DD') AND TO_DATE('2022-12-29', 'YYYY-MM-DD')
SELECT status, id, created_at FROM customers WHERE updated_at BETWEEN TO_DATE('2021-09-10', 'YYYY-MM-DD') AND TO_DATE('2021-09-26', 'YYYY-MM-DD') AND name IN ('yivrm', 'txqys', 'eftsc', 'qfkak', 'ciafn', 'nnffl', 'qnffz', 'qlprg', 'pgwix', 'lmaac') AND created_at BETWEEN TO_DATE('2021-12-10', 'YYYY-MM-DD') AND TO_DATE('2022-01-11', 'YYYY-MM-DD')
SELECT status, id, created_at FROM products WHERE name IN ('afcpy', 'obytw', 'nmjvg') AND id >= 23 AND created_at BETWEEN TO_DATE('2021-10-18', 'YYYY-MM-DD') AND TO_DATE('2022-08-19', 'YYYY-MM-DD') AND updated_at BETWEEN TO_DATE('2022-06-21', 'YYYY-MM-DD') AND TO_DATE('2021-04-25', 'YYYY-MM-DD') AND status IN ('dqvaq', 'jfdfd', 'ezosn', 'gslay', 'sfixg', 'jevhq', 'kdnhl', 'bnnca', 'jkxlv')
```
