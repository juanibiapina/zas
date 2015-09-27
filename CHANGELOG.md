# Changelog

### master

- Change installation to use `brew`
- Remove old install using `curl`
- Add `list` command
- Add `link` command
- Add `unlink` command

### 0.12.0

- Add install and uninstall commands

### 0.11.0

- Add zas cli (not available when installing using curl)
- Add command to restart zas

### 0.10.0

- Enable colors in logs
- Wrap install and uninstall scripts in curly braces to prevent partial
  execution
- Rename main binary to zasd

### 0.9.0

- Properly terminate child process
- Handle error when unable read user home directory
- Handle error when parsing port environment variables

### 0.8.0

- Run the proper command to terminate apps
- Wait for terminanting app to finish

### 0.7.0

- Add route to terminate apps
- Change to rust nightly in order to use Child pid
