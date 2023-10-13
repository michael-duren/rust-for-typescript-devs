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
