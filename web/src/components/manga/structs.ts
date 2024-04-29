export type Lang = "en" | "jp";

export interface Manga {
  title: string;
  lang: Lang;
  img: number[];
}

export interface LangOptions {
  en: boolean;
  jp: boolean;
}

export interface Filters {
  langs: LangOptions;
}

export const ImageError = "ERROR";
