import "./App.css";
import { createBrowserRouter, RouterProvider } from "react-router-dom";
import ViewManga from "./components/manga/ViewManga";
import Header from "./components/Header/Header";
import Footer from "./components/Footer/Footer";
import Home from "./components/Home/Home";
import ViewSingleManga from "./components/manga/ViewSingleManga";
import ViewPage from "./components/manga/ViewPage";

function App() {
  const router = createBrowserRouter([
    {
      path: "/",
      element: <Home />,
    },
    {
      path: "/manga",
      element: <ViewManga />,
    },
    {
      path: "/manga/:lang/:title",
      element: <ViewSingleManga />,
    },
    {
      path: "/manga/:lang/:title/:volume/:page",
      element: <ViewPage />,
    },
  ]);

  return (
    <>
      <Header />
      <RouterProvider router={router} />
      <Footer />
    </>
  );
}

export default App;
