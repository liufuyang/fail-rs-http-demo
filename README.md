# fail-rs-http-demo
Run and play like:
```sh
FAILPOINTS=home=panic cargo run
curl localhost:8080
# Working. Try: curl http://localhost:8080/failpoints/index -XPUT -d'panic'

curl http://localhost:8080/failpoints/index -XPUT -d'panic'
# Add fail point with name: index, actions: panic

curl localhost:8080
# curl: (52) Empty reply from server

curl localhost:8080/failpoints
# home: panic
# index: panic

curl localhost:8080/failpoints/index -XDELETE
# Delete fail point with name: index

curl localhost:8080/failpoints
# home: panic

curl localhost:8080
# Working. Try: curl http://localhost:8080/failpoints/index -XPUT -d'panic'
```