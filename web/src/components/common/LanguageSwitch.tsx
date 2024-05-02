import { useEffect, useState } from "react";
import { Link as RouterLink, useParams } from "react-router-dom";
import { Lang } from "../manga/structs";

interface Props {
  path: string;
}

const LanguageSwitch = ({ path }: Props) => {
  const { title, lang, page, volume } = useParams();
  const [langPath, setLangPath] = useState<Lang>();
  const [label, setLabel] = useState<string>();

  useEffect(() => {
    setLangPath(lang === "en" ? "jp" : "en");
    setLabel(lang === "en" ? "Japanese" : "English");
  }, [lang]);


  return (
    <RouterLink to={`/${path}/${title}/${langPath}/${volume}/${page}`}>
      {label}
    </RouterLink>
  );
};

export default LanguageSwitch;
