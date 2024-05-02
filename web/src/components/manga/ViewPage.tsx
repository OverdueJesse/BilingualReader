import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Link as RouterLink } from "react-router-dom";
import ImageNotFound from "./ImageNotFound";
import { ImageError, Page, VolumeMetadata } from "./structs";
import LanguageSwitch from "../common/LanguageSwitch";

const ViewPage = () => {
  const { lang, title, page, volume } = useParams();
  const pageNum = Number(page);
  const [url, setUrl] = useState<string>();
  const [metadata, setMetadata] = useState<VolumeMetadata>({ page_count: -1 });

  useEffect(() => {
    const getManga = async () => {
      try {
        const data: Page = await axios
          .get(
            `${
              import.meta.env.VITE_API_URL
            }/manga/${title}/${lang}/${volume}/${page}`
          )
          .then((res) => res.data);
        if (data.img?.length) {
          setUrl(
            window.URL.createObjectURL(
              new Blob([new Uint8Array(data.img).buffer])
            )
          );
          setMetadata(data.metadata);
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
        <RouterLink to={`/manga/`}>Back To Manga</RouterLink>
      </div>
      <LanguageSwitch path="manga" />
      <div>
        <img src={url} style={{ height: "85vh", width: "auto" }} />
      </div>
      {pageNum > 0 && (
        <RouterLink to={`../manga/${title}/${lang}/${volume}/${pageNum - 1}`}>
          Previous
        </RouterLink>
      )}
      {metadata.page_count > Number(page) ? (
        <>
          <RouterLink to={`../manga/${title}/${lang}/${volume}/${pageNum + 1}`}>
            Next
          </RouterLink>
        </>
      ) : (
        <>
          <RouterLink to={`../manga/${title}/`}>
            Home
          </RouterLink>
        </>
      )}
    </>
  );
};

export default ViewPage;
