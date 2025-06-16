# pid-gpt

![Static Badge](https://img.shields.io/badge/Platforms-Linux,_macOS-forestgreen?style=for-the-badge)
[![built with garnix](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fgarnix.io%2Fapi%2Fbadges%2FCollegiate-Edu-Nation%2Fpid-gpt%3Fbranch%3Dmain&style=for-the-badge&color=grey&labelColor=grey)](https://garnix.io/repo/Collegiate-Edu-Nation/pid-gpt)
![Static Badge](https://img.shields.io/badge/Powered_by_Nix-grey?logo=nixOS&logoColor=white&logoSize=auto&style=for-the-badge)

Agentic proof-of-concept for leveraging local LLMs to tune PID controllers (like cruise control)

Written in Rust and adapted from the [Rig example]

<img width="674" alt="Image" src="https://storage.googleapis.com/pid_gpt_img/PID_tuner.png" />

## Usage

_Ensure Ollama is installed (w/ Mistral downloaded) and running_

This project leverages Nix so the simplest approach for running it is

```shell
nix run github:collegiate-edu-nation/pid-gpt
```

Leverage the binary cache by adding [Garnix] to your nix-config

```nix
nix.settings.substituters = [ "https://cache.garnix.io" ];
nix.settings.trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
```

You can also just install Rust and Cargo yourself and run it after cloning

```shell
cargo run
```

Keep in mind this is just a proof-of-concept and that Mistral doesn't always play nice with tool calls, so expect a few tries before all 10 iterations succeed

## License

[GPLv3]

[Rig example]: https://github.com/0xPlaygrounds/rig-examples/blob/75d5060397e6247eecc7a637fc0291171c08de94/pid_controller_tuner_example/src/main.rs
[Garnix]: https://garnix.io/
[GPLv3]: COPYING
