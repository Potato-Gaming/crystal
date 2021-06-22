@echo off
setlocal
  echo Creating database
  docker run -it --rm --name crystal-api -e POSTGRES_PASSWORD=postgres -p 5432:5432 postgres

endlocal
