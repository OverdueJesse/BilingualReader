@ECHO OFF
CD "%~dp0/api"
START cargo run
CD "../web"
START npm run dev
EXIT