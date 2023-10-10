import fs from "fs";

const arg: string | undefined = process.argv[2];

if (arg) {
  try {
    const data = fs.readFileSync(arg, "utf8");
    data
      .toString()
      .split("\n")
      .forEach((line) => console.log(line));
  } catch (error) {
    console.error(error);
  }
} else {
  console.log("please include file path");
}
