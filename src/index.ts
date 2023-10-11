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

interface Area {
  aera(): number;
}

class Rectangle implements Area {
  constructor(
    public x: number,
    public y: number,
    public width: number,
    public height: number,
  ) {}
  aera(): number {
    return this.width * this.height;
  }
}

class Circle implements Area {
  /**
   *
   */
  constructor(
    public x: number,
    public y: number,
    public radius: number,
  ) {}
  aera(): number {
    return this.radius * this.radius * Math.PI;
  }
}
