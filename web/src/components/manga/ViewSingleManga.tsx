import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { VolumeList, Lang } from "./structs";
import { Link as RouterLink } from "react-router-dom";

const ViewSingleManga = () => {
  const { title } = useParams();
  const [volumes, setVolumes] = useState<VolumeList>();

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(
        `${import.meta.env.VITE_API_URL}/manga/${title}`
      );
      console.log(res.data);
      setVolumes(res.data);
    };
    getManga();
  }, [title]);

  return (
    <>
      {volumes &&
        Object.keys(volumes).map((lang, i) => {
          return (
            <div key={`${lang} volumes ${i}`}>
              <strong>{lang === "en" ? "English" : "日本語"}</strong>
              {volumes[lang as Lang].map((v, l) => {
                return (
                  <div>
                    <RouterLink to={`${lang}/${l}/0`} key={`Volume ${i}`}>
                      {v.title}
                    </RouterLink>
                  </div>
                );
              })}
            </div>
          );
        })}
    </>
  );
};

export default ViewSingleManga;
