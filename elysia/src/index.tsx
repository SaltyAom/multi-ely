import { Elysia, t } from "elysia";
import { Html } from "@elysiajs/html";

const Component = ({ name }: { name: string }) => (
  <html>
    <head>
      <title>Hello</title>
      <meta charset="utf8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <style>{`
        html, body {
          margin: 0;
          font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
          -webkit-font-smoothing: antialiased;
          -moz-osx-font-smoothing: grayscale;
        }
      `}</style>
    </head>
    <body>
      {/* Escape HTML */}
      <h1 safe>Hi {name}</h1>
      <img src="/ely.png" alt="Herrscher of Human" />
    </body>
  </html>
);

const app = new Elysia()
  .get("/", "hi")
  .post("/json", ({ body }) => body, {
    body: t.Object({
      name: t.String(),
    }),
  })
  .get("/id/:id", ({ set, query: { name }, params: { id } }) => {
    set.headers["x-powered-by"] = "benchmark";

    return id + " " + name;
  })
  .get("/ely.png", Bun.file("public/ely.png"))
  .get("/page.html", ({ query: { name }, set }) => {
    set.headers['content-type'] = 'text/html'

    return <Component name={name} />
  }, {
    query: t.Object({
      name: t.String()
    })
  })
  .listen(3000);
