export type Lang = "en" | "jp";

export interface Manga {
  title: LangOptions;
  thumbnail: number[];
  volume_count: number;
}

export interface Volume {
  title: string;
}

export interface VolumeList {
  en: Volume[],
  jp: Volume[],
}

export interface LangOptions {
  en: boolean;
  jp: boolean;
}

export interface Filters {
  langs: LangOptions;
}

export const ImageError = "ERROR";
