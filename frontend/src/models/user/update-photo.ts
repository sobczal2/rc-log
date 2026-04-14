export interface UpdateUserPhotoDto {
  id: string;
  username: string;
  email: string;
  photoAssetId?: string | null;
}
