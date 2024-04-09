export interface Manga {
  title: string;
  description: string;
}

export type Lang = "en" | "jp" | "all";

export interface LangOptions {
  en: boolean;
    jp: boolean;
}

export interface Filters {
  langs: LangOptions;
}
