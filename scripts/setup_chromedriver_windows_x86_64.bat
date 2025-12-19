@echo off
REM Downloads latest chromedriver (Windows x86_64) for local testing

setlocal enabledelayedexpansion

for /f "delims=" %%v in ('powershell -Command "Invoke-WebRequest -UseBasicParsing https://chromedriver.storage.googleapis.com/LATEST_RELEASE | Select-Object -ExpandProperty Content"') do set CHROME_DRIVER_VERSION=%%v
set DRIVER_URL=https://chromedriver.storage.googleapis.com/%CHROME_DRIVER_VERSION%/chromedriver_win32.zip

powershell -Command "Invoke-WebRequest -Uri %DRIVER_URL% -OutFile chromedriver.zip"
powershell -Command "Expand-Archive -Path chromedriver.zip -DestinationPath . -Force"
del chromedriver.zip

echo chromedriver.exe downloaded.
