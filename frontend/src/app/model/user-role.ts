export class LogIn {
  private email: string;
  private password: string;

  constructor(email: string, password: string) {
    this.email = email;
    this.password = password;
  }
}


export enum UserRole {
  UNAUTHORIZED,
  ROLE_ADMINISTRATOR,
  ROLE_STAFF
}
