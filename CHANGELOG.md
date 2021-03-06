# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Releases]

## [1.0.0] - 2022-02-19

### Gunfollower v1.0 Release

#### Features

- Now you can also track people you are following from `gunfollower`!
- You can add `p10k battery`, use the command `gunfollower shell` to learn more

#### Contributors

@DarthBenro008

## [0.1.4] - 2022-01-06

### Enable Gunfollower default command and enhanced CI

#### Features

- Now you can simply run `gunfollower` instead of `gunfollower check`
- Automatic CI to bump and update Homebrew releases

## [0.1.3] - 2021-12-08

### Nullable Fields fix of various user data

#### Features

- Patch to datatype of some fields to be optional of GitHub API of various field

## [0.1.2] - 2021-12-05

### Hireable Field data-type mismatch bug fix

#### Features

- Patch to datatype of GitHub API of the Hireable field from String to bool

## [0.1.1] - 2021-12-04

### Homebrew Support Patch Release

#### Features

- Patch to `releaser.yaml` CI to support binary builds that can be published to homebrew by @DarthBenro008

## [0.1.0] - 2021-12-04

### Initial Release

#### Features

- Use to know who followed and unfollowed you on GitHub
- Uses sled db
- Caching Mechansim
- Smart status and logging status
