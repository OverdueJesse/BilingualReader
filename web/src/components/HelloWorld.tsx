import { useEffect, useState } from "react";
import { apiTest } from "../types/manga";
import axios from "axios";

const HelloWorld = () => {
  const [test, setTest] = useState<apiTest>();

  useEffect(() => {
    testFunc();
  }, [])

  const testFunc =  async () => {
    const res = await axios.get(`${import.meta.env.VITE_API_URL}/manga`)
    setTest(res.data);
    console.log(res.data);
  }

  return (
    <div>{test && test.title}</div>
  )
}

export default HelloWorld