# Contributing on Windows
## By ambiguousname
Windows tends to be more neglected when it comes to development stuff, so I'm writing mostly to note for myself the common pitfalls I'm encountering while building things.

For reference, I'm using msys2 with C:\msys64\usr\bin in my PATH (plus some C:\mingw64\ folders) to try and avoid some bash-specific complications. For rust though, that doesn't mean all that much.

If you get frustrated, you could always try switching to WSL. Which I try not to use because I'm difficult.

## Path issues
The way windows parses paths will be your downfall. You can always try \\ or /, and I'd say for a specific build step it helps to try and write out the full path that you want to use rather than relying on the script variables to merge everything together.

Sometimes it's as simple as a directory vs. a file. Windows seems to see `some/directory/path/FolderName` as a file in duckscript, so adding `some/directory/path/FolderName/` seems to help. Not sure if this breaks things for other OSes though, so that will be something to test.

## Environment issues
### Missing NPM
Per https://stackoverflow.com/questions/78242352/why-i-get-program-not-found-error-on-running-npm-v-command-with-rust-command, `npm.exe` is not a command on Windows. `npm.ps1` or `npm.cmd` is, however.

If you need to quickly test something, you can replace whatever script is calling `npm` with `npm.cmd`. 

The permanent solution depends on where you're seeing npm being run from. If it's Rust, see the above post. For `Makefile.toml`s using duckscript, it's tricky.

The main thing to do is to see if it's a script mostly moving files around and running things that the user should have installed. If so, I'd change it to an `@shell` script.

For full windows support, it'd be nice to have `npm` as a variable that switches based on OS. You can add this line for a windows-exclusive fix (but it breaks on other systems):

```
npm = which npm
```

You also don't have to use Cargo make. VS Code tasks also help me circumvent this kind of stuff. But I try to avoid calling npm directly if possible, since everybody should ideally be using the same build tools.

## CRLF vs. LF
### Random Whitespacing in Generation (core.autocrlf)
Git, in its attempts to be helpful, created a bunch of extra whitespaces in converting CRLF to LF. Which led to differences from the CI on Windows.

So these commands help get rid of that:

```bash
# Disable Git's CRLF to LF conversion:
git config --global core.autocrlf false
# Reset cached files to remove associations
git rm --cached -r .
git reset --hard
```

You can re-enable the `core.autocrlf` option afterwards if you'd like: `git config --global core.autocrlf true`. Just don't reset the cache.