import axios from "axios";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { Link as RouterLink } from "react-router-dom";

const ViewPage = () => {
  const { lang, title, page, volume } = useParams();
  const pageNum = Number(page);
  const [url, setUrl] = useState<string>();

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(
        `${
          import.meta.env.VITE_API_URL
        }/manga/${lang}/${title}/${volume}/${page}`
      );
      console.log(res.data);
      setUrl(
        window.URL.createObjectURL(new Blob([new Uint8Array(res.data).buffer]))
      );
    };
    getManga();
  }, [lang, title, page, volume]);

  return (
    <>
      {url && (
        <>
          <img src={url} />
          {pageNum > 0 && (
            <RouterLink
              to={`../manga/${lang}/${title}/${volume}/${pageNum - 1}`}
            >
              Previous
            </RouterLink>
          )}
          <RouterLink to={`../manga/${lang}/${title}/${volume}/${pageNum + 1}`}>
            Next
          </RouterLink>
        </>
      )}
    </>
  );
};

export default ViewPage;
