export interface GetByIdUserDto {
  id: string;
  username: string;
  email: string;
  photoAssetId?: string | null;
}
