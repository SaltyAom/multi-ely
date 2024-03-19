# multi ely
Benchmark throughput between frameworks.
- Elysia
- Axum
- Actix

## Prerequisted
- Bun
- Rust
- wrk (apt install wrk)

## Setup
You are expected to run benchmark manually, and open 2 terminal.

1. Start HTTP server in either:
- elysia
```bash
cd elysia && bun src/spawn.ts
```

- axum
```bash
cd axum && cargo run --release
```

- actix
```bash
cd actix && cargo run --release
```

2. On the second terminal, run the benchmark command using wrk:
```bash
cd scripts && chmod +x ./wrk.sh && ./wrk.sh
```

## Test
See `scripts/wrk.sh`

The condition for the following case:
- Hi:
    - bare bone simple hello world
    - path: '/'
    - method: 'GET'
    - expected response: 'hi'
    - nothing fancy, prove nothing
- Query & Params:
    - Parse query and path params
    - path: '/id/{id}?name={string}
    - method: 'GET'
    - expected path input: /id/1?name=bun
    - expected response: `1 bun`
    - expected headers:
        - x-powered-by: benchmark
    - how fast router can handle common utility
    - note: query maybe optional
- JSON:
    - parse incoming and serialize outgoing JSON
    - path: '/json'
    - method: 'POST'
    - expected body: '{"name": "saltyaom" }'
    - expected response: '{"name": "saltyaom" }'
    - must deserialize body as JSON, and serialize the body as string to return to client
    - must perform body validation
- Image:
    - return an image file to client
    - path: '/ely.png'
    - method: 'GET'
    - expected response: Binary of file in `public/ely.png`
        - an PNG file size of 2.65MB
    - measure maximum outgoing throughput
- Template Engine
    - using template engine to render html page with data from query
    - path: '/page.html'
    - method: 'GET'
    - expected response: an HTML page with value from query
    - expected query:
        - name: hello
    - expected query for XSS test:
        - name: Ely<script>XSS</script>
    - must perform query validation
    - must perform input sanitization to prevent XSS attack

## Additional note
There's an automate test using `bench.ts` but since it allocate some port beforehand, the benchmark will not perform in full performance as expected.
