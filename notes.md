# Testing with netcat

Occupy port: `netcat -kl 8080`

Write to server: `echo "test 1 2 3" | netcat 127.0.0.1 8080`
