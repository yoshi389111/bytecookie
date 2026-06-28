# BYTECOOKIE(6)

## NAME

bytecookie - display engineer-themed fortune messages

## SYNOPSIS

`bytecookie [OPTIONS]`

## DESCRIPTION

`bytecookie` displays random fortune messages for engineers, inspired by HTTP status codes, programming idioms, error messages, and developer culture.

When a username is specified, a deterministic message is selected based on the current date and username.

Small bytes. Small bites. Small pieces of wisdom.

## OPTIONS

- `-u`, `--user <USER>`
  - Specify a username to get the same message throughout the day.

- `-j`, `--json <PATH>`
  - Load fortune messages from the specified JSON file. If not provided, embedded messages are used.

- `-c`, `--color <WHEN>`
  - Control colored output. Options are:
    - `auto` (default): Enable color if output is a terminal.
    - `always`: Always enable color.
    - `never`: Never enable color.

## ENVIRONMENT

- `BYTE_COOKIES_JSON`  
  Path to a custom message JSON file. Used if the `--json` option is not specified.

- `NO_COLOR`  
  If set, disables colored output unless overridden by command-line options. See [NO_COLOR](https://no-color.org/) for more details.

## EXAMPLES

```sh
# Get a random engineer fortune
bytecookie

# Get today's fortune for user 'alice@example.com'
bytecookie --user alice@example.com

# Use a custom message file
bytecookie --json ./mycookies.json
```

## SAMPLE OUTPUT

```txt
202 Accepted
You've been acknowledged. Processing... eventually.
```

```txt
418 I'm a teapot
Don't take things too seriously. Be playful.
```

```txt
sudo !!
You knew better. But not soon enough.
```

```txt
git commit -m "final final v2 REALLY final"
Some things are never truly finished.
```

```txt
"It works on my machine"
Your reality is not universal.
```

```txt
// HACK: works for now
Shortcuts often lead to scenic detours later.
```

```txt
fatal: not a git repository
You might be in the wrong place... or not initialized yet.
```

## SEE ALSO

fortune(6), cowsay(1)

## COPYRIGHT

&copy; 2025 SATO Yoshiyuki. MIT Licensed.
