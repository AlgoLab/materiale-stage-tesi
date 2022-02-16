# bioconda

Come creare un pacchetto conda e caricarlo su bioconda (guida creata sulla base di https://bioconda.github.io/contributor/index.html)

```
# 1) creare una fork di https://github.com/bioconda/bioconda-recipes

# 2) clonare la fork e aggiornarla
git clone <repo>
git remote add upstream https://github.com/bioconda/bioconda-recipes.git
git checkout master
git pull upstream master
git push origin master

# 3) creare un branch per il nuovo pacchetto
git checkout -b {recipe-name}

# 4) creare/modificare una recipe (nel caso di una nuova recipe, partire da una recipe esistente e modificarne il contenuto)
cd recipes
mkdir {recipe-name}
cd {recipe-name}
# 4a) creare/modificare il file build.sh, script di compilazione/installazione
# 4b) creare/modificare il file meta.yaml, contenente le informazioni generali sulla recipe
# 4c) nel caso di un aggiornamento, ricordarsi di incrementare il numero di build

# 5) testare la recipe
cd ../..
./bootstrap.py /tmp/miniconda
source ~/.config/bioconda/activate
bioconda-utils lint --packages {recipe-name}
bioconda-utils build [–docker] --mulled-test --packages {recipe-name}
# Attenzione: se uno di questi comandi dovesse fallire, leggere qui https://github.com/bioconda/bioconda-utils/issues/642#issuecomment-600884997

# 6) commit&push
git commit
git push --set-upstream origin {recipe-name}

# 7) Creare una pull request (via github)
# 7bis) Nel caso di un aggiornamento, è possibile pushare direttamente verso l'upstream, git push upstream {recipe-name}
```

### Note
Ho testato il tutto qualche mese fa, bisogna controllare che i comandi siano ancora validi. Appena avrò necessità di creare/aggiornare un pacchetto, controllo.
