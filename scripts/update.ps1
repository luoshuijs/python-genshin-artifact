Invoke-WebRequest -Uri "https://raw.githubusercontent.com/EnkaNetwork/API-docs/refs/heads/master/store/characters.json" -OutFile "characters.json"
Move-Item -Force "characters.json" "python_genshin_artifact/enka/assets/characters.json"