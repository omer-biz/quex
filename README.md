# Quex

Quex is a simple command-line tool that functions like a calendar, inspired by
[When](http://www.lightandmatter.com/when/when.html). Just type `quex` in the
command line to get a list of upcoming events.

Events can be stored in any text file. By default, Quex looks for a `quex` block
in Markdown files or files with the `.quex` extension. For `.quex` files, no
specific block is needed.

```quex
jan 1 2025, Play soccer with the boys.
```

Quex supports both Ethiopian and Gregorian dates (details below).

## Quick Start

Make sure you have Rust installed.

1. Clone the repository:
   ```shell
   git clone https://github.com/omer-biz/quex
   cd quex
   cargo build
   ```

2. Create an `example.quex` file with events:

   ```quex
   jan 1 2025, Play soccer with the boys.
   mes 1 *, ኩዋስ ከጀለሶች ጋር
   q=5, ክፊያ
   jan 1 1990*, My best friend's birthday. He was born in \y and now he is \a years old.
   ```

3. Run Quex to display the events:
   ```shell
   cargo run -F eth -- --quex="./example.quex" all
   ```

Expected output:
   ```shell
   January 1, 2024; My best friend's birthday. He was born in 1990 and now he is 34 years old.
   መስከረም 01, 2017; ኩዋስ ከጀለሶች ጋር
   ኅዳር 05, 2017; ክፊያ
   January 1, 2025; Play soccer with the boys.
   ```

To output in JSON format:
   ```shell
   cargo run -F eth -- --quex="./example.quex" --format=json all 2>/dev/null | jq .
   ```
   
   
  ``` json
  [
    {
      "description": "My best friend's birthday. He was born in 1990 and now he is 34 years old.",
      "diff": -315,
      "date": "January 1, 2024"
    },
    {
      "description": "ኩዋስ ከጀለሶች ጋር",
      "diff": -61,
      "date": "መስከረም 01, 2017"
    },
    {
      "description": "ክፊያ",
      "diff": 3,
      "date": "ኅዳር 05, 2017"
    },
    {
      "description": "Play soccer with the boys.",
      "diff": 51,
      "date": "January 1, 2025"
    }
  ]
  ```
   

The `diff` field in JSON shows the difference in days from today to the event date.

> **Note**: Running without `-F eth` may cause parsing errors if Ethiopian dates are included.


## Syntax

### Yearly

Yearly:

```quex
jan 1 *, Play soccer with the boys.
```

This will create an annual event replacing `*` with the current year.

Named Yearly:

```quex
jan 1 1990*, My best friend's birthday. He was born in \y and now he is \a years old.
```

This will change into:

```
January 1, 2024; My best friends birthday. He was born in 1990 and now he is 34 years old.
```

### Monthly

```quex
d=5, Pay bills.
```

Every month on day 5 of the month, remindes you to pay the bills.

### Ethiopan Date

You have to enable the `eth` feature.

``` quex
mes 1 *, ኩዋስ ከጀለሶች ጋር
```

or

``` quex
መስከ 1 *, ኩዋስ ከጀለሶች ጋር
```

or monthly

``` quex
q=5, ክፊያ
```

If you want to use the Ethiopian calender, change the months to ethiopian
months available choices are:

| English Letters | Amharic Letters | Ethiopian Month |
| --------------- | --------------- | --------------- |
| mes             | መስከ             | መስከረም           |
| tik             | ጥቅም             | ጥቅምት            |
| hed             | ህዳር             | ህዳር             | 
| tah             | ታኅሣ             | ታኅሣሥ            | 
| tir             | ጥር              | ጥር              | 
| yek             | የካቲ             | የካቲት            | 
| meg             | መጋቢ             | መጋቢት            | 
| miy             | ሚያዝ             | ሚያዝያ            | 
| gin             | ግንቦ             | ግንቦት            | 
| sen             | ሴኒ              | ሴኒ              | 
| ham             | ሐምሌ             | ሐምሌ             | 
| neh             | ነሐሴ             | ነሐሴ             | 
| pua             | ጳጉሜ             | ጳጉሜ             | 

## Configuration

The configuration is done via a simple file.

This is what they look like with their default value.

```toml
calendar = "/home/user/.config/quex/calendar/"
editor = "nvim"
# future = 
# past =
# format = 
```


quex loads configs in this order `cli arg -> env variable -> config file`.


### calender

This is the folder where quex look for files to parse if you don't give any path with `--quex` argument.


The calendar can be set to a single file or a directory of files.
if you have multiple file types and you want `quex` to parse them
you should use the `--file-format` and `--block` arguments. like this

``` shell
quex --file-format=org --block='#+begin_src quex,#+end_src' \
     --file-format=txt --block='quex_begin,quex_end'
```

### editor

Your text editor of choice to open the `calendar` when you run `quex e[dit]`.


### future

How far future events you want to be printed. default 14 days.

### past

How late past events you want to be printed. default 3 days.

### format

What format would you like the events to be printed, there is json
and plain text. iCal coming soon.


## Cli Arguments

```txt
Usage: quex [OPTIONS] [COMMAND]

Commands:
  edit   edit calendar file
  week   view schedules file for the week
  month  view schedules file for the month
  year   view schedules file for the year
  all    view schedules file for all time
  help   Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>            path to config file
  -q, --quex <QUEX>                path to calendar file
  -e, --editor <EDITOR>            command to open calendar file
  -f, --future <FUTURE>            How many days into the future the report extends [default: 14]
  -p, --past <PAST>                How many days into the past the report extends [default: 3]
      --format <FORMAT>            Specify the format to use for printing the schedules [default: plain] [possible values: json, plain]
      --filter <FILTER>            Filter using a sub string
      --date-window <DATE_WINDOW>  Filter by window of time
      --file-format <FILE_FORMAT>  File format (e.g., md, org) [default: md]
      --block <BLOCK>              Block start and end (e.g., ```quex,```) [default: ```quex,```]
  -h, --help                       Print help
```


### `--date-windw`

this has a couple of different formats:

- Absolute snippets

Print events from jan 1, 2002 to today.
``` shell
quex --date-window="2002:jan:1"
```

Print events from jan 1, 2002 to jan 2, 2003.
``` shell
quex --date-window="2002:jan:1,2003:jan:1" 
```

- Relative snippets

Print events from jan 1 this year upto today.
``` shell
quex --date-window="jan:1"
```

Print events from jan 1 to feb 1 this year.
``` shell
quex --date-window="jan:1,feb:1"
```
