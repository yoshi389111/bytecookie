# BYTECOOKIE(6)

## NAME

bytecookie - A variation of the Fortune command for engineers

## SYNOPSIS

`bytecookie [OPTIONS]`

## DESCRIPTION

`bytecookie` is a command-line tool that displays a random or daily "fortune" message for engineers, inspired by HTTP status codes, programming idioms, and error messages. Messages are designed to be motivational, humorous, or thought-provoking for developers.

Messages are selected either randomly or deterministically based on the current date and an optional user name, ensuring a unique "fortune" for each user per day.

## OPTIONS

- `-u`, `--user <USER>`
  - Specify a user name to receive a daily fortune message. The same user will get the same message throughout the day.

- `-j`, `--json <PATH>`
  - Specify a custom JSON file containing fortune messages. If not provided, embedded messages are used.

## ENVIRONMENT

- `BYTE_COOKIES_JSON`  
  Path to a custom message JSON file. Used if the `--json` option is not specified.

## EXAMPLES

```sh
# Get a random engineer fortune
bytecookie

# Get today's fortune for user 'alice@example.com'
bytecookie --user alice@example.com

# Use a custom message file
bytecookie --json ./mycookies.json
```

## FILE

- Custom message file (JSON format, see `assets/bytecookies.json` for example)

## SEE ALSO

fortune(6), cowsay(1)

## COPYRIGHT

&copy; 2025 SATO Yoshiyuki. MIT Licensed.
