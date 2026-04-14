export interface UpdateUserRequest {
  newUsername: string;
}

export interface UpdateUserDto {
  id: string;
  username: string;
  email: string;
  photoAssetName?: string | null;
}
