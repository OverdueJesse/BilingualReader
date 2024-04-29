# PortfolioWebsite

 Bat file will start terminals for both the API and WEB project.

 ### API
  `cargo run`

 ### WEB
  `npm i`
  
  `npm run dev`

  ### Notes

  Okay so this tiny project is just filling a personal gap in my everyday life. Or that's why I started it at least.

  In short: Rust backend is given a root folder where my manga is stored. From this it pulls titles, thumbnails, language, to send to the front end. Front end is very crude but for now it's essentially just a skeleton to give the backend purpose. All images are converted to byte arrays on the backend then blobbed on the front end and resized.
