{ pkgs ? import <nixpkgs> {} }:
  pkgs.mkShell {
      enableDebugging = true;
      buildInputs = with pkgs; [
        rustup
        lldb
        rust-analyzer
        llvmPackages_latest.llvm
        llvmPackages_latest.bintools
        llvmPackages_latest.lld 
        bash
      ];
}