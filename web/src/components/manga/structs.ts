export type Lang = "EN" | "JP";

export interface Manga {
  title: string;
  lang: Lang;
  img: number[];
  pages?: number;
}

export interface Volume {
  title: string;
}

export interface LangOptions {
  EN: boolean;
  JP: boolean;
}

export interface Filters {
  langs: LangOptions;
}

export const ImageError = "ERROR";
