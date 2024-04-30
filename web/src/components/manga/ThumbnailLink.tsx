import { Link as RouterLink } from "react-router-dom";
import { Manga } from "./structs";
import Thumbnail from "./Thumbnail";

interface Props {
  manga: Manga;
}

const ThumbnailLink = ({ manga }: Props) => {
  console.log(manga);
  return (
    <>
      <RouterLink to={`${manga.title.en}`}>
        <Thumbnail blob={new Blob([new Uint8Array(manga.thumbnail).buffer])} />
      </RouterLink>
    </>
  );
};

export default ThumbnailLink;
