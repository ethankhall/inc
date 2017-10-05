# `inc-checkout`

This command provides an easy way to retrieve repos. The repo can be any of the supported types (see below), and how you determine the sources is pluggable.

## Normal Usecase
For users that just want to check things out from GitHub, all you need to run is `inc checkout {project}/{repo}`. To checkout `inc` you would run `inc checkout ethankhall/inc`.

## Custom Usecase
For users that have custom ways to determine how to checkout a project, you may need something extra. This extra bit is called a 'service' in `inc` termonology. A service takes in user input and generates a URL to checkout. To do this, `inc` will execute the service with one argument, the user input and expect that the URL be the only thing printed onto STDOUT and the exit code will be 0.

If the exit code is non 0, or the URL isn't understood `inc` will stop checking out and report the error to the user.

For `inc` to understand these services, they must be an executable on the `PATH` and follow the naming scheme `inc-checkout-service-{service name}`. The service name will automatically show up in the help. To use the custom service you would use the `--service` option, like `inc checkout --service=crom ethankhall/gradle-plugins`.

The script for the `crom` service would look like:

```
#!/bin/sh

IFS='/' read -r -a array <<< "$1"

length=${#array[@]}

if [[ length -ne 2 ]]; then
  >&2 echo 'You must specify input in the form foo/bar'
  exit 1
fi

curl http://api.crom.tech/api/v1/project/${array[0]}/repo/${array[1]} | jq -r .url
```

This script takes input, splits it by /, validates that the input is correct, and then makes REST and parses out the URL from the response.

The custom service as you can see can be written in any langugage that is needed.

## Details

### Supported Services
- GitHub
- BitBucket
- VCS

### Supported SCM Systems:
- git
- svn (future)
- hg (future)