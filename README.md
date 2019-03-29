# fail-rs-http-demo
Run and play like:
```sh
FAILPOINTS=home=panic cargo run
curl localhost:8080
# Working. Try: curl localhost:8080/fail -XPOST -d'{"name": "index", "actions": "panic"}'

curl localhost:8080/fail -XPUT -d'{"name": "index", "actions": "panic"}'
# Add fail point with name: index, actions: panic

curl localhost:8080
# curl: (52) Empty reply from server

curl localhost:8080/fail
# home: panic
# index: panic

curl localhost:8080/fail -XDELETE -d'{"name": "index"}'
# Delete fail point with name: index

curl localhost:8080/fail
# home: panic

curl localhost:8080
# Working. Try: curl localhost:8080/fail -XPOST -d'{"name": "index", "actions": "panic"}'
```