import { useEffect, useState } from "react";
import { apiTest } from "../types/manga";
import axios from "axios";

const HelloWorld = () => {
  const [test, setTest] = useState<apiTest>();

  useEffect(() => {
    testFunc();
  }, [])

  const testFunc =  async () => {
    const res = await axios.get(`${import.meta.env.VITE_API_URL}`)
    setTest(res.data);
  }

  return (
    <div>{test && test.description}</div>
  )
}

export default HelloWorld