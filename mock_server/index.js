const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Access-Control-Allow-Origin": "*" });

  // Handle /boards/:slug route
  if (req.url.startsWith('/boards/')) {
    res.end(JSON.stringify({
      name: "Random",
      category: "Misc.",
      slug: "a",
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
      { name: "Anime & Manga", category: "anime", slug: "a" },
      { name: "Random", category: "Misc.", slug: "b" },
      { name: "ROBOT9001", category: "Misc.", slug: "r9k" },
    ])
  );
});

server.listen(3000, () => {
  console.log("Server is running on port 3000");
});
