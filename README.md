Key - ssh for dummies


# key - easy ssh 
Make ssh easier on Mac, super hard right now

[init - mdBook Documentation](https://rust-lang.github.io/mdBook/cli/init.html)

- [SSH (Secure Shell) - Wikipedia](https://en.wikipedia.org/wiki/SSH_(Secure_Shell))
- [clap::App - Rust](https://docs.rs/clap/2.33.3/clap/struct.App.html)
- [GitHub - clap-rs/clap: A full featured, fast Command Line Argument Parser for Rust](https://github.com/clap-rs/clap#using-yaml)

### GitHub
- [Generating a new SSH key and adding it to the ssh-agent - GitHub Docs](https://docs.github.com/en/free-pro-team@latest/github/authenticating-to-github/generating-a-new-ssh-key-and-adding-it-to-the-ssh-agent)

### GitLab
- [GitLab and SSH keys | GitLab](https://docs.gitlab.com/ee/ssh/)

### Use
- `ssh-keygen`
- `ssh-agent`
- `ssh-add`

### Examples
```shell
$ key generate --type <TYPE-NAME> <KEY-NAME> --email "your_email@example.com"
  Generated <KEY-NAME> keys
  Fingerprint sha256 is <FINGERPRINT>
  Added <KEY-NAME> to ssh-agent
  Copied public key <KEY-NAME> to your clipboard.

$ key generate --github
  Copied public key to your clipboard.
  Please paste on GitHub.

$ key generate --gitlab
  Copied public key to your clipboard.
  Please paste on GitLab.

$ key check git@gitlab.com
  Succefully connected to GitLab!

$ key check git@github.com
  Succefully connected to GitHub!

$ key exists --type <TYPE-NAME> --name <KEY-NAME>
  Does not exist.

$ key list
  Type	     Creation Date		Key name
  ----       ------------- 		--------
  rsa								id_rsa
  ecdsa								id_ecdsa
  ed25519							id_ed25519

$ key copy <KEY-NAME>
  Copied public key <KEY-NAME> to your clipboard.

$ key copy --private <KEY-NAME>
  WARNING: Copied private key <KEY-NAME> to your
           clipboard.

$ key info --type <TYPE-NAME>
  Information about the type...
```

#dev/cli
