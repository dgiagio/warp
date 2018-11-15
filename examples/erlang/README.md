You need to have [`Erlang/OTP`](http://erlang.org), [`Rebar3`](http://rebar3.org) (Erlang build tool) and `warp-packer` installed to create self-contained binary Erlang release.

## Linux
Create new application using `rebar3`: 
```sh
~ $ rebar3 new app foo && cd foo
```
```txt
===> Writing foo/src/foo_app.erl
===> Writing foo/src/foo_sup.erl
===> Writing foo/src/foo.app.src
===> Writing foo/rebar.config
===> Writing foo/.gitignore
===> Writing foo/LICENSE
===> Writing foo/README.md
```
```sh
~/foo $ 
```

Make a directory (e.g. `config`) to place `warp` launcher in it:  
```sh
~/foo $ mkdir config
```
Create `./config/launch` file with below contents:
```sh
#!/bin/sh

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
APP=$DIR/bin/{{release_name}}

exec $APP $@
```
We used [`mustache`](https://mustache.github.io/) template to let `rebar3` place your release name in script dynamically. Here it is `foo`. 

Make launcher file executable:
```sh
~/foo $ chmod a+x config/launch
```

Edit `rebar.config` file and add `relx` section:
```erlang
% ...

{relx, [{release, { foo, "0.1.0" }, [foo, sasl]},

        {dev_mode, false},
        {include_erts, true}, % To include Erlang runtime system
        {extended_start_script, true}, % Should be true

        {overlay, [{template, "./config/launch", "launch"}]}] % copies our launcher file to release directory
}.

% ...
```

Make a release:
```sh
~/foo $ rebar3 release
```
```text
===> Verifying dependencies...
===> Compiling foo
===> Starting relx build process ...
===> Resolving OTP Applications from directories:
          /opt/foo/_build/default/lib
          /usr/local/lib/erlang/lib
          /opt/foo/_build/default/rel
===> Resolved foo-0.1.0
===> Including Erts from /usr/local/lib/erlang
===> release successfully created!
```
Now launcher file should be in your release directory:
```sh
~/foo $ ls _build/default/rel/foo 
bin  erts-10.1  launch  lib  releases
```

Before running `warp-packer`, It's better to test release itself:
```sh
~/foo $ ./_build/default/rel/foo/bin/foo start
~/foo $ ./_build/default/rel/foo/bin/foo ping
pong
~/foo $ ./_build/default/rel/foo/bin/foo stop
ok
```

Build self-contained binary file from release:
```sh
~/foo $ warp-packer -a linux-x64 -i _build/default/rel/foo -e launch -o foo
```
```text
Compressing input directory "_build/default/rel/foo"...
Creating self-contained application binary "foo"...
All done
```

Test it:
```sh
~/foo $ ./foo start
```
```sh
~/foo $ ./foo ping
pong
```
```sh
~/foo $ ./foo remote_console
Erlang/OTP 21 [erts-10.1] [source] [64-bit] [smp:4:4] [ds:4:4:10] [async-threads:1] [hipe]

Eshell V10.1  (abort with ^G)
(foo@localhost)1>
BREAK: (a)bort (c)ontinue (p)roc info (i)nfo (l)oaded
       (v)ersion (k)ill (D)b-tables (d)istribution
```
```sh
~/foo $ ./foo stop
ok
```
```sh
~/foo $ ./foo ping
Node foo@localhost not responding to pings.
```

#### Advanced
You can place `warp` command in `rebar.config`. Then `rebar3` will run it when you are making release:
```erlang
% ...

{post_hooks, [{release, "warp-packer -a linux-x64 -i _build/default/rel/foo -e launch -o foo"}]}.

% ...
```
Now make release:
```sh
~/foo $ rebar3 release
```
```text
===> Verifying dependencies...
===> Compiling foo
===> Starting relx build process ...
===> Resolving OTP Applications from directories:
          /opt/foo/_build/default/lib
          /usr/local/lib/erlang/lib
          /opt/foo/_build/default/rel
===> Resolved foo-0.1.0
===> Including Erts from /usr/local/lib/erlang
===> release successfully created!
Compressing input directory "_build/default/rel/foo"...
Creating self-contained application binary "foo"...
All done
```

Note that `rebar3` uses a templating approach to make an application structure. If you built `rebar3`by yourself, and did not use its escript, Its own templates are placed in `/path/to/installed/rebar3/priv/templates`.
You can place your `launch` file there and add below lines at the end of `app.template`:
```erlang
% ...
{file, "launch", "{{name}}/config/launch"}. % copies launch file to YourApp/config/launch
{chmod, "{{name}}/config/launch", 8#755}. % makes file executable
```
Also edit `app_rebar.config` and place our `relx` and `post_hooks` at the end of file:
```erlang
% ...

{relx, [{release, { {{name}}, "0.1.0" },
         [{{name}}, sasl]},

        {dev_mode, false},
        {include_erts, true},

        {extended_start_script, true},
        {overlay, [{template, "./config/launch", "launch"}]}]
}.

{post_hooks, [{release, "warp-packer -a linux-x64 -i _build/default/rel/{{name}} -e launch -o {{name}}"}]}.
```
In above `{{name}}` is what you will pass to `rebar3 new app` as application name. Recompile `rebar3` itself again: 
```sh
/path/to/rebar3 $ ./bootsrtap
...
```

Create new application again:
```sh
~ $ rebar3 new app bar && cd bar
```
```text
===> Writing bar/src/bar_app.erl
===> Writing bar/src/bar_sup.erl
===> Writing bar/src/bar.app.src
===> Writing bar/rebar.config
===> Writing bar/.gitignore
===> Writing bar/LICENSE
===> Writing bar/README.md
===> Writing bar/config/launch
```
Make a release:
```sh
~/bar $ rebar3 release
```
```text
===> Verifying dependencies...
===> Compiling bar
===> Starting relx build process ...
===> Resolving OTP Applications from directories:
          /opt/bar/_build/default/lib
          /usr/local/lib/erlang/lib
===> Resolved bar-0.1.0
===> Including Erts from /usr/local/lib/erlang
===> release successfully created!
Compressing input directory "_build/default/rel/bar"...
Creating self-contained application binary "bar"...
All done
```

Test it:
```sh
~/bar $ ./bar start
```
```sh
~/bar $ ./bar ping
pong
```
```sh
~/bar $ ./bar stop
ok
```

Also you can add it for `rebar3 new release | lib` or [build your own template](https://www.rebar3.org/docs/using-templates#section-custom-templates) and add it to your template.
