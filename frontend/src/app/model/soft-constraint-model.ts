export class SoftConstraintModel {
  constructor(
    public id: string,
    public prefers: prefers,
  ) {
  }
}

export class prefers {
  constructor(
    public monday: number,
    public tuesday: number,
    public wednesday: number,
    public thursday: number,
    public friday: number) {
  }
}
