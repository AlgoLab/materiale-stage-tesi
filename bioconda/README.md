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

# 5) testare la recipe - see https://bioconda.github.io/contributor/building-locally.html
mamba create -n bioconda -c conda-forge -c bioconda bioconda-utils
conda activate bioconda
bioconda-utils lint --git-range master
bioconda-utils build --docker --mulled-test --git-range master

# 6) commit&push
git commit
git push --set-upstream origin {recipe-name}

# 7) Creare una pull request (via github)
# 7bis) Nel caso di un aggiornamento, è possibile pushare direttamente verso l'upstream, git push upstream {recipe-name}
```
