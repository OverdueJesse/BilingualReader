import axios from "axios";
import { useEffect, useState } from "react";
import { Filters, LangOptions, Manga } from "./structs";
import Checkbox from "@mui/material/Checkbox";
import { FormControlLabel, FormGroup } from "@mui/material";
import ThumbnailLink from "./ThumbnailLink";

const defaultFilters = {
  langs: {
    en: true,
    jp: false,
  },
};

const ViewManga = () => {
  const [filters, setFilters] = useState<Filters>(defaultFilters);
  const [manga, setManga] = useState<Manga[]>([]);

  useEffect(() => {
    const getManga = async () => {
      const res = await axios.get(
        `${import.meta.env.VITE_API_URL}/manga`
      );
      setManga(res.data);
    };

    getManga();
  }, []);

  return (
    <div>
      <FormGroup>
        {["en", "jp"].map((l, i) => {
          const checkbox = (
            <Checkbox
              checked={filters.langs[l as keyof LangOptions]}
              onChange={(e) => {
                const newFilters = { ...filters };
                newFilters.langs[l as keyof LangOptions] = e.target.checked;
                setFilters(newFilters);
              }}
              inputProps={{ "aria-label": "controlled" }}
            />
          );

          return (
            <FormControlLabel
              key={`Lang Filter ${i}`}
              control={checkbox}
              label={l === "en" ? "English" : "Japanese"}
            />
          );
        })}
      </FormGroup>

      {manga.map((m, i) => {
        if (filters.langs[m.lang]) {
          return <ThumbnailLink manga={m} key={`${m.title} ${i}`}/>;
        }
      })}
    </div>
  );
};

export default ViewManga;
