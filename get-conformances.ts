// deno run --allow-net ./get-conformances.ts

import * as ink from "https://deno.land/x/ink@1.3/mod.ts";
import { readLines, StringReader } from "https://deno.land/std@0.86.0/io/mod.ts";
const THE_SPEC = "https://solidproject.org/TR/protocol";

const res = await fetch(THE_SPEC);

//read line-by-line (I assume there's a cleaner way to do this, but can't find it)
const specReader = new StringReader(await res.text());

for await (let specLine of readLines(specReader)) {
  const matchMust = specLine.match(/(?!client)\b[^"]MUST\b/);
  if (matchMust) {
    console.log(convertHtml(specLine));
  }
}

function convertHtml(htmlString: String) {
  return htmlString
      .replace("<p>", "\n")
      .replace("</p>", "")
}
