import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Volume } from "./structs";
import { Link as RouterLink } from "react-router-dom";

const ViewSingleManga = () => {
  const { lang, title } = useParams();
  const [volumes, setVolumes] = useState<Volume[]>([]);

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(
        `${import.meta.env.VITE_API_URL}/manga/${lang}/${title}`
      );
      console.log(res.data);
      setVolumes(res.data);
    };
    getManga();
  }, [lang, title]);

  return (
    <>
      {volumes.map((volume, i) => {
        return (
          <div>
            <RouterLink to={`${volume.title}/${0}`} key={`Volume ${i}`}>
              {volume.title}
            </RouterLink>
          </div>
        );
      })}
    </>
  );
};

export default ViewSingleManga;
