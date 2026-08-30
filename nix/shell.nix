{pkgs, ...}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
  ];

  shellHook = ''
    echo "Welcome to the acmcsufoss bot shell. Happy coding!"
    echo -e "rustc:\t ${pkgs.rustc.version}"
    echo -e "cargo:\t ${pkgs.cargo.version}"
  '';
}
