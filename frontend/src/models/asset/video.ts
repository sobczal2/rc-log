export interface VideoPathsDto {
  name: string;
  smallPath: string;
  mediumPath?: string;
  largePath?: string;
}

export function getVideoUrl(smallPath: string): string {
  return `/api/assets/${smallPath}`;
}
