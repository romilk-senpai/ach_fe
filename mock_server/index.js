const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Access-Control-Allow-Origin": "*" });

  // Handle /boards/:slug route
  if (req.url.startsWith('/boards/')) {
    const slug = req.url.split('/').pop();
    res.end(JSON.stringify({
      category_id: 1,
      id: 1,
      name: "Random",
      slug: slug,
      description: "Random board",
      threads: [
        {
          id: "1",
          num: 1,
          name: "~sonter-rabryd",
          subject: "Test Thread",
          content: "This is a test thread",
          timestamp: 1716150000,
          board: "a"
        }
      ]
    }));
    return;
  }

  // Default route - return boards list
  res.end(
    JSON.stringify([
      { category_id: 1, id: 1, name: "Anime & Manga", slug: "a" },
      { category_id: 2, id: 2, name: "Random", slug: "b" },
      { category_id: 2, id: 3, name: "ROBOT9001", slug: "r9k" },
    ])
  );
});

server.listen(3000, () => {
  console.log("Server is running on port 3000");
});
