import { Link as RouterLink } from "react-router-dom";
import { Manga } from "./structs";

interface Props {
  manga: Manga;
}

const ThumbnailLink = ({ manga }: Props) => {
  return (
    <div>
      <RouterLink to={`${manga.lang}/${manga.title}`}>{manga.title}</RouterLink>
    </div>
  );
};

export default ThumbnailLink;
