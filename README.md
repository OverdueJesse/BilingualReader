# Bilingual Reader

 Bat file will start terminals for both the API and WEB project.

 ### API
  `cargo run`

 ### WEB
  `npm i`
  
  `npm run dev`

  ### Notes

  Just a little idea for a bilingual reader with some built in dictionary and kanji search functionality. I haven't really seen this done right and I know the community around learning Japanese wants it.

  In short: Rust backend is given a root folder where my manga is stored. From this it pulls titles, thumbnails, language, to send to the react front end. Front end is very crude but for now it's essentially just a skeleton to give the backend purpose. All images are converted to byte arrays on the backend then blobbed on the front end and resized.
