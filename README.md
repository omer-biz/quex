# Quex

Quex is some what a calender inspired by
[when](http://www.lightandmatter.com/when/when.html). The basic idea, just like
when, is to type `quex` at the command line and get a list of uppcomming events

The events are stored in a markdown file inside a quex block.

```quex
2025 jan 1, Play soccer with the boys.
```

When parsing the markdown file `quex` will ignore everything else but the `quex` block.

It supports to dates Ethiopian and Gregorian.

## Syntax

### Yearly

Yearly:

```quex
* jan 1, Play soccer with the boys.
```

This will create an event every year replacing the `*` with the current year.

Named Yearly:

```quex
1990* jan 1, My best friends birthday. He was born in \y and now he is \a years old.
```

This will change into:

```
2024 jan 1, My best friends birthday. He was born in 1990 and now he is 34 years old.
```

### Monthly

```quex
d=5, Pay bills.
```

Every month on day 5 of the month, remindes you to pay the bills.

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

Recurring monthly for ethiopian date is not supported for now.

## Configuration

The configuration is done via a simple toml file. For now there are only two options.

This is what they look like with their default value.

```toml
calendar = "/home/user/.config/quex/calendar/"
editor = "nvim"
```

The `calendar` directory is where the markdown files are stored.

The calendar can be set to a single file or a directory of markdown files where each of them
have a `quex` block.

You will be asked, if the `calendar` directory doesn't exists, if you want to create it.

## Cli Arguments

...snip...

Passing `--future` and `--past` won't take effect if one of the following commands are passed.

- `week`
- `month`
- `year`
- `all`
