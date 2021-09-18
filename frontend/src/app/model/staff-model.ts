import {UserRole} from "./user-role";

export class StaffModel {
  public password = "123qweasd"
  public role = UserRole.ROLE_STAFF

  constructor(
    public id: string,
    public email: string,
    public name: string,
    public surname: string,
    public lectures: string[],
    public faculty: string,
  ) {
  }
}
