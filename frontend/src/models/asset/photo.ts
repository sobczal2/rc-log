export interface PhotoPathsDto {
  id: string;
  smallPath: string;
  mediumPath?: string;
  largePath?: string;
}

export function getPhotoUrl(path: string): string {
  return `/api/assets/${path}`;
}
