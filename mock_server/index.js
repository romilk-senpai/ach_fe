const http = require("http");

const server = http.createServer((req, res) => {
  res.writeHead(200, { "Access-Control-Allow-Origin": "*" });

  // Handle /boards/:slug route
  if (req.url.startsWith('/boards/')) {
    res.end(JSON.stringify({}));
    return;
  }

  // Default route - return boards list
  res.end(
    JSON.stringify([
      { name: "Anime & Manga", category: "anime", handle: "a" },
      { name: "Random", category: "Misc.", handle: "b" },
      { name: "ROBOT9001", category: "Misc.", handle: "r9k" },
    ])
  );
});

server.listen(3000, () => {
  console.log("Server is running on port 3000");
});
