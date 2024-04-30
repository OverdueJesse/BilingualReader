import axios from "axios";
import { useEffect, useState } from "react";
import { Manga } from "./structs";
import ThumbnailLink from "./ThumbnailLink";

const ViewManga = () => {
  const [manga, setManga] = useState<Manga[]>([]);

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(`${import.meta.env.VITE_API_URL}/manga`);
      setManga(res.data);
    };

    getManga();
  }, []);

  return (
    <div>
      {manga.map((m, i) => {
        return <ThumbnailLink manga={m} key={`${m.title} ${i}`} />;
      })}
    </div>
  );
};

export default ViewManga;
