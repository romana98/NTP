import {UserRole} from "./user-role";

export class LogInModel {
  constructor(
    private email: string,
    private accessToken: string,
    private role: string
  ) {
  }

  getRole(): UserRole {
    return this.role === 'ROLE_ADMINISTRATOR' ? UserRole.ROLE_ADMINISTRATOR : (this.role === 'ROLE_STAFF' ? UserRole.ROLE_STAFF : UserRole.UNAUTHORIZED);
  }
}
