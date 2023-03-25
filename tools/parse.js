const fs = require("fs");
const path = require("path");

const target_path = path.join(__dirname, "../prompts");

const content = fs.readFileSync("prompts.csv").toString();

const items = content.split("\n").slice(1);

const index = [];
const prompts = [];

items.forEach((item) => {
  let [act, ...promptChunk] = item.split(",");
  const prompt = promptChunk.join(",").slice(1, -1);
  act = act.slice(1, -1).replace(/\//g, '|');

  index.push({
    act,
  });

  prompts.push({
    act: act,
    prompt,
  });
});

fs.writeFileSync(
  path.join(target_path, "index.json"),
  JSON.stringify(index, null, 2)
);

prompts.forEach((item) => {
  fs.writeFile(
    path.join(target_path, 'data', `${item.act}.json`),
    JSON.stringify(item, null, 2),
    (err) => {
      if (err) {
        console.log(err);
      }
    }
  );
});
