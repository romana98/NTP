export class FacultyModel {
  constructor(
    public id: string,
    public name: string,
    public hard_constraint: string,
    public shifts: string[],
    public staff: string[],
    public lectures: string[],
    public schedule: string
  ) {
  }
}
