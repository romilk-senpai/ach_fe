const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Access-Control-Allow-Origin": "*" });
  // Handle POST request to /post
  if (req.method === "POST" && req.url === "/post") {
    res.end("OK");
    return;
  }

  // Handle /boards/:slug route
  if (req.url.startsWith("/board?slug=")) {
    const [_, _slug, _id] = req.url.split("=");
    const slug = _slug.replace("&", "");
    const id = parseInt(_id?.replace("&", "")) || 0;
    if (id) {
      res.end(
        JSON.stringify({
          lastReplies: [
            {
              id: id + 1,
              author: "ayanokojimode",
              subject: "topic 1",
              content: "sample post 1",
              createdAt: "2025-05-18T15:41:20.936326Z",
            },
            {
              id: id + 2,
              author: "",
              subject: "topic 2",
              content: "sample post 2",
              createdAt: "2025-05-18T15:41:20.936326Z",
            },
          ],
          opPost: {
            id: id,
            author: "",
            subject: "Test Thread",
            content: "This is a test thread",
            createdAt: "2025-05-18T15:41:20.936326Z",
          },
        })
      );
      return;
    }
    res.end(
      JSON.stringify({
        categoryName: "Misc.",
        id: 1,
        name: "Random",
        slug: slug,
        description: "Random board",
        threads: [
          {
            lastReplies: [
              {
                id: 2,
                author: "ayanokojimode",
                subject: "topic 1",
                content: "sample post 1",
                createdAt: "2025-05-18T15:41:20.936326Z",
              },
              {
                id: 3,
                author: "ayanokojimode",
                subject: "topic 2",
                content: "sample post 2",
                createdAt: "2025-05-18T15:41:20.936326Z",
              },
            ],
            opPost: {
              id: 1,
              author: "",
              subject: "Test Thread",
              content: "This is a test thread",
              createdAt: "2025-05-18T15:41:20.936326Z",
            },
          },
        ],
      })
    );
    return;
  }

  // Default route - return boards list
  res.end(
    JSON.stringify([
      { categoryName: "Japanese Culture", id: 1, name: "Anime & Manga", slug: "a" },
      { categoryName: "Misc.", id: 2, name: "Random", slug: "b" },
      { categoryName: "Misc.", id: 3, name: "ROBOT9001", slug: "r9k" },
    ])
  );
});

const port = 8080;

server.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
