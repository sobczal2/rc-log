import type { UserDto } from "@/models/__generated/user/update";

export interface UpdateUserRequest {
  newUsername: string;
}

export type UpdateUserDto = UserDto;
