const http = require("http");

const generateRandomName = () => {
  return Math.random().toString(36).substring(2, 15);
};

const generateRandomSubject = () => {
  return Math.random().toString(36).substring(2, 15);
};

const generateRandomContent = () => {
  return Math.random().toString(36).substring(2, 15);
};

const generateRandomCreatedAt = () => {
  return new Date(Date.now() - Math.random() * 1000 * 60 * 60 * 24).toISOString();
};

const generateRandomOpPost = () => {
  return {
    id: Math.floor(Math.random() * 1000000),
    author: generateRandomName(),
    subject: generateRandomSubject(),
    content: generateRandomContent(),
    createdAt: generateRandomCreatedAt(),
  };
};

const generateRandomLastReplies = (length) => {
  return Array.from({ length }, (_, i) => ({
    id: i + 1,
    author: "",
    subject: generateRandomSubject(),
    content: generateRandomContent(),
    createdAt: generateRandomCreatedAt(),
  }));
};

const generateRandomThreads = (length) => {
  return Array.from({ length }, (_, i) => ({
    lastReplies: generateRandomLastReplies(Math.floor(Math.random() * 4)),
    opPost: generateRandomOpPost(),
  }));
};

const opPost = {
  id: 1,
  author: "",
  subject: "topic 1",
  content: ">creating an image board in 2025 \n >mfw \n Here's a %%spoiler%% \n and **bold** and *italic* and __underline__",
  createdAt: "2025-05-18T15:41:20.936326Z",
};

const lastReplies = [
  ...generateRandomLastReplies(9).filter((_, i) => i !== 0),
  {
    id: 9,
    author: "ayanokojimode",
    subject: "topic 1",
    content: "sample post 1",
    createdAt: "2025-05-18T15:41:20.936326Z",
  },
  {
    id: 10,
    author: "",
    subject: "topic 2",
    content: ">>2 \n >>3 \n >sample post 2",
    createdAt: "2025-05-18T15:41:20.936326Z",
  },
];

const thread = {
  lastReplies: lastReplies,
  opPost: opPost,
};

const server = http.createServer((req, res) => {
  console.log(req.url);
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
      res.end(JSON.stringify(thread));
      return;
    }
    res.end(
      JSON.stringify({
        categoryName: "Misc.",
        id: 1,
        name: "Random",
        slug: slug,
        description: "Random board",
        threads: generateRandomThreads(5),
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
