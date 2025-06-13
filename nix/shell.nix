{ pkgs, deps }:

{
  default = pkgs.mkShell {
    packages = deps.dev;

    shellHook = ''
      echo -e "\npid-gpt DevShell via Nix Flake\n"

      echo -e "┌─────────────────────────┐"
      echo -e "│     Useful Commands     │"
      echo -e "├─────────────────────────┤"
      echo -e "│ Build    │ $ build      │"
      echo -e "│ Format   │ $ format     │"
      echo -e "│ Run      │ $ cargo run  │"
      echo -e "│ Test     │ $ cargo test │"
      echo -e "└──────────┴──────────────┘"
    '';
  };
}
