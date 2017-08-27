# Changelog

### 0.18.0

- Rename main binary to zas
- Remove process management
- Remove system daemon
- Remove bash CLI
- Update to rust 1.19
- Update http proxy to use async hyper

### 0.17.0

- Add zas term command
- Add zas tail command
- Update to rust 1.7

### 0.16.0

- Setup automatic releases on travis and github

### 0.15.0

- Allow apps with dots in the name

### 0.14.0

- Use full path to user shell on launch agent

### 0.13.0

- Change installation to use `brew`
- Remove old install using `curl`
- Add `list` command
- Add `link` command
- Add `unlink` command
- Fix bug on install when /etc/resolver folder doesn't exist
- Add zasd log output

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
