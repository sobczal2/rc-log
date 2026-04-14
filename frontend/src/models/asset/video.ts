export interface VideoPathsDto {
  id: string;
  smallPath: string;
  mediumPath?: string;
  largePath?: string;
}

export function getVideoUrl(smallPath: string): string {
  return `/api/assets/${smallPath}`;
}
