import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Link as RouterLink } from "react-router-dom";
import ImageNotFound from "./ImageNotFound";
import { ImageError } from "./structs";

const ViewPage = () => {
  const { lang, title, page, volume } = useParams();
  const pageNum = Number(page);
  const [url, setUrl] = useState<string>();

  useEffect(() => {
    const getManga = async () => {
      try {
        const res = await axios.get(
          `${
            import.meta.env.VITE_API_URL
          }/manga/${lang}/${title}/${volume}/${page}`
        );
        console.log(res);
        if (res.data?.length) {
          setUrl(
            window.URL.createObjectURL(
              new Blob([new Uint8Array(res.data).buffer])
            )
          );
        } else {
          throw new Error("Bad blob");
        }
      } catch (e) {
        setUrl(ImageError);
      }
    };
    getManga();
  }, [lang, title, page, volume]);

  return url === ImageError ? (
    <ImageNotFound />
  ) : (
    <>
      <div>
        <img src={url} style={{ height: "85vh", width: "auto" }} />
      </div>
      {pageNum > 0 && (
        <RouterLink to={`../manga/${lang}/${title}/${volume}/${pageNum - 1}`}>
          Previous
        </RouterLink>
      )}
      <RouterLink to={`../manga/${lang}/${title}/${volume}/${pageNum + 1}`}>
        Next
      </RouterLink>
    </>
  );
};

export default ViewPage;
