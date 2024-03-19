wrk -c 16384 -d 15s -t 6 http://localhost:3000/
wrk -c 16384 -d 15s -t 6 'http://localhost:3000/id/1?name=bun'
wrk -c 16384 -d 15s -t 6 -s scripts/body.lua http://localhost:3000/json
wrk -c 16384 -d 15s -t 6 http://localhost:3000/ely.png
wrk -c 16384 -d 15s -t 6 'http://localhost:3000/page.html?name=hello'
