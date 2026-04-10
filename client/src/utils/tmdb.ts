const TMDB_IMAGE_BASE = "https://image.tmdb.org/t/p/";

export const getImageUrl = (path: string | null, size: "w500" | "original" = "w500") => {
  if (!path) return "/no-image.png";
  return `${TMDB_IMAGE_BASE}${size}${path}`;
};
