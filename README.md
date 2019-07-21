# prettyjson
prettyjson - command line JSON format tool.


```
$ prettyjson --help
Usage:
  prettyjson [OPTIONS] [FILE]

prettyjson - command line JSON pretty tool.

Positional arguments:
  FILE                  FILE

Optional arguments:
  -h,--help             Show this help message and exit
  -w,--tab-width TAB_WIDTH
                        Tab width, default 4
  -v,--version          Print version
```

```
$ echo '{"k": "value"}' | prettyjson
{
    "k": "value"
}
```


