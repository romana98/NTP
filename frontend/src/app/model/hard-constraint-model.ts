export class HardConstraintModel {
  constructor(
    public id: string,
    public daily_max: number,
    public weekly_max: number,
    public weekly_min: number,
    public max_per_shift: number
  ) {
  }
}
