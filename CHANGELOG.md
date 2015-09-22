# Changelog

### master

- Enable colors in logs
- Wrap install and uninstall scripts in curly braces to prevent partial
  execution

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
