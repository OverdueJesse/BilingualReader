import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Manga } from "./structs";

const ViewSingleManga = () => {
  const { lang, title } = useParams();
  const [manga, setManga] = useState<Manga>();

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(
        `${import.meta.env.VITE_API_URL}/manga/${lang}/${title}`
      );
      setManga(res.data);
    };
    getManga();
  }, [lang, title]);

  return <div>{title}</div>;
};

export default ViewSingleManga;
