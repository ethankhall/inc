# etrain

How on the CI train.

## What
Etrain is a pluggable tool that allows people to use the same tool across multiple projects.

By default etrain contains two different commands out of the box.

### Checkout
`etrain checkout ethankhall/etrain` will do a git clone on the github repo and pull it down. There will be more options in
the future, but at the moment github is only supported.

This tool should also support other SCMs.

### Exec - TODO
`etrain exec build` will lookup the build command in etrain.yaml. This allows teams to specify any arbitrary commands and have it shared consistantly across the team.