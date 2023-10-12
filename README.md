# WEM
**WEM** (short for WirEase iMproved) is a simple file generating program written is Rust.

-----------
## How to use it
> You can see more information in wiki on my website.

- **Basic uses**

**Create a filesystem**

Use `make` subcommand. set your wem script's name to the first value.
```sh
wem make your-script
```

> note that, if you have `%NAME%` in your wem script,
> you can set your project name as a second value.
> ```sh
> wem make your-script your_project
> ```

**List all the available wem scripts**

Use `list` subcommand.

```sh
wem list
```

- **Options** 


**Specify the file, from which WEM loads wem scripts**

With both `list` and `make`, you can set the source directory where the wem scripts are stored in with `-s`/`--source`.
```sh
wem make/list -s path/to/your/directory
```


**Format Time/Date**

With only `make`, you can set how the `%TIME%` will be formatted using -t/--time option.
```sh
wem make your-wem-script your_project -t "%Y-%m-%d"
```

> Available tags:<br />
>   %Y: year <br />
>   %m: month <br />
>   %d: day <br />
> The time format is based on `chrono` crate.
> For further information, see the official repo on `chrono`


**Specify Where To Output The Files**

With only `make`, you can set the output with -o/--output option.
```sh
wem make your-wem-script your_script_name -o path/to/the/output
```


**Specify The Configuration File**

```sh
wem -c path/to/your/config make/list ...
```


**Set Mode**

```sh
wem -m testdebugtime make/list ...
```

> Available modes:<br />
>   test:  when set , WEM will not create any actuall files.<br />
>   debug: display internal values. useful when you have a problem with parsing<br />
>   time:  display benchmark<br />


## License
it's not licensed at all for now
