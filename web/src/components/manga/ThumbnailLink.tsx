import { Link as RouterLink } from "react-router-dom";
import { Manga } from './structs';

interface Props {
  manga: Manga;
  key: number
}

const ThumbnailLink = ({manga, key}: Props) => {
  return (
    <div key={`Manga ${key}`}>
      <RouterLink to={`${manga.lang}/${manga.title}`}>{manga.title}</RouterLink>
    </div>
  )
}

export default ThumbnailLink