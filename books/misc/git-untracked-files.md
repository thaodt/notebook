# How do I make Git forget about a file that was tracked, but is now in .gitignore?
`.gitignore` will prevent untracked files from being added (without an `add -f`) to the set of files tracked by Git. However, Git will continue to track any files that are already being tracked.


To stop tracking a file, we must remove it from the index:
```bash
git rm --cached <file>
```

To remove a folder and all files in the folder recursively:
```bash
git rm -r --cached <folder>
```
then: 
```bash
git add .
```