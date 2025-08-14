# Changelog

## 0.1.0 (2025-08-14)


### ⚠ BREAKING CHANGES

* Application restructured as library with multiple interfaces

### Features

* Add gcc to devShell for Rust compilation ([9538028](https://github.com/shift/garnix-insights/commit/953802826b22e27d42b912219c1e145099f5a76f))
* Add generated Cargo.lock ([c1c2dc3](https://github.com/shift/garnix-insights/commit/c1c2dc3e4cdb15ea76491e73980890b0f64dde08))
* Add JSON output option to garnix-fetcher ([53653e9](https://github.com/shift/garnix-insights/commit/53653e95e6e9319ceaa551db6cf1bab4438c45c5))
* Add packages.default output to flake.nix ([9945976](https://github.com/shift/garnix-insights/commit/9945976688e617b947fa0cd1253499496c2949c9))
* add release-please automation ([afd49c3](https://github.com/shift/garnix-insights/commit/afd49c377823a2b936830bf7db5124bb49aeded6))
* Align flake.nix with crane working example ([afd1ffb](https://github.com/shift/garnix-insights/commit/afd1ffb946595727816504d2d04d183e672ea059))
* configure Garnix cache integration and remove Cachix dependency ([eabe777](https://github.com/shift/garnix-insights/commit/eabe7776094613bfa1b2fcfefa88a4d2cbf28d06))
* configure Garnix cache integration and remove Cachix dependency ([968c163](https://github.com/shift/garnix-insights/commit/968c163fc7500228c2a123a476db48ac63c19922))
* Define CLI and server apps in flake.nix ([fe17bd7](https://github.com/shift/garnix-insights/commit/fe17bd7064dd11a7684820a1f6c4c1385e9d041e))
* Implement server mode in garnix-fetcher ([d9fde00](https://github.com/shift/garnix-insights/commit/d9fde00d203a87a5cb162b4fd6bbbbfed5ad4d75))
* Integrate server logic into garnix-fetcher ([f768ca9](https://github.com/shift/garnix-insights/commit/f768ca99fd68123401743f0bf0762e3f5b12ded6))
* transform monolithic application into professional library ([61c78a4](https://github.com/shift/garnix-insights/commit/61c78a4bbee07ba4288393d7225a6b6f8f76b0ef))
* transform to garnix-insights with multi-mode support ([0f905db](https://github.com/shift/garnix-insights/commit/0f905dbedd8be73c87dd81614645e444dce01af9))
* Update Cargo.lock with actix-web dependencies ([67620b5](https://github.com/shift/garnix-insights/commit/67620b5554259b4a05e4051afc28630c5be5e090))
* Update Cargo.lock with anyhow dependency ([da4f80d](https://github.com/shift/garnix-insights/commit/da4f80dc4f9a31983e35f16d1be20b8f17245e8f))
* Use crane for building Rust package ([1773fc9](https://github.com/shift/garnix-insights/commit/1773fc9954584b41a4d5adf448dae44485691f8a))
* Use pkgs.callPackage for structured dependency management ([d4750ae](https://github.com/shift/garnix-insights/commit/d4750ae4c1ed57a430163f870dfaac4834d9c920))


### Bug Fixes

* Access crane.lib directly in flake.nix ([09a0931](https://github.com/shift/garnix-insights/commit/09a09319020f4ff322dcfd057a8ff280425bbc38))
* add pkg-config and openssl to buildInputs in flake.nix ([a981b08](https://github.com/shift/garnix-insights/commit/a981b08d8a5db8f73cec775d66074e78fe2ef0ab))
* Add pkg-config and openssl to buildInputs in flake.nix ([e97c972](https://github.com/shift/garnix-insights/commit/e97c972c65ebc7f41c5a551bdf1317848bff58e6))
* async block return types and error handling in main function ([1284d59](https://github.com/shift/garnix-insights/commit/1284d590e758e0f9a791e7beadf25e5d1898a3e3))
* async block syntax with proper type annotations ([895e2a0](https://github.com/shift/garnix-insights/commit/895e2a0650b8806e9d6f47c1a334a04b285bf917))
* Correct cargoLock path in flake.nix ([0fff43a](https://github.com/shift/garnix-insights/commit/0fff43aade4cdf36cea943baba5a2b9d92f9232c))
* crane input and update Cargo.lock ([c477116](https://github.com/shift/garnix-insights/commit/c4771163d50b33110f63097e9e1cef0a789e7147))
* Ensure async blocks return Result for ? operator ([515bca0](https://github.com/shift/garnix-insights/commit/515bca04a756261b055b6f9727c614eaa0726692))
* Ensure async blocks return Result for ? operator ([fdc06f0](https://github.com/shift/garnix-insights/commit/fdc06f000147d7d424e85238d2785383256cdc62))
* Ensure async blocks return Result for ? operator ([fd1da1f](https://github.com/shift/garnix-insights/commit/fd1da1fc25e6a627420fb9fdb3552462a510e341))
* Ensure async blocks return Result for ? operator ([d24740c](https://github.com/shift/garnix-insights/commit/d24740cfd2d5350c37e0a2db3e6d312495a14a43))
* Ensure async blocks return Result for ? operator ([ea8fbc5](https://github.com/shift/garnix-insights/commit/ea8fbc59573f61b99e9b1452fa6abb47db6016c5))
* Ensure async blocks return Result for ? operator ([824b63d](https://github.com/shift/garnix-insights/commit/824b63dbefc3ac46107b13677afbfc37049dffde))
* Ensure async blocks return Result for ? operator ([7233b2e](https://github.com/shift/garnix-insights/commit/7233b2e26bb618f20ff38829307e0a45adb9fe25))
* Explicitly define async block return types for ? operator ([f43ee5e](https://github.com/shift/garnix-insights/commit/f43ee5e5c61b2c106fec94c34985bc97af3aff26))
* Explicitly define async block return types for ? operator ([504622e](https://github.com/shift/garnix-insights/commit/504622e9be8aa17e39ae571bbb869183cb658816))
* Explicitly define async block return types for ? operator ([056b8cd](https://github.com/shift/garnix-insights/commit/056b8cd552bffaf76349ccdb9a99e795b784deb8))
* Explicitly link OpenSSL libraries in flake.nix ([fba4e2b](https://github.com/shift/garnix-insights/commit/fba4e2b82c11c1f7fd26c34423ac385487714d89))
* Explicitly set OpenSSL env vars in crane build ([de7137f](https://github.com/shift/garnix-insights/commit/de7137f538736d22427d9c3b08b36191db1e768f))
* Explicitly set OPENSSL_DIR in flake.nix ([1a652e1](https://github.com/shift/garnix-insights/commit/1a652e12402b9197dc32dd0846211afacde53ed8))
* Make main synchronous and use tokio runtime for async blocks ([4d30cd2](https://github.com/shift/garnix-insights/commit/4d30cd2755492ffa4402a30f67547142991a71d7))
* Remove --release from cargoBuildFlags in flake.nix ([ba1b7b5](https://github.com/shift/garnix-insights/commit/ba1b7b51b6fb23b75a986c17c7fdf8d591410c61))
* Remove crane.inputs.nixpkgs.follows from flake.nix ([52ba27f](https://github.com/shift/garnix-insights/commit/52ba27f02edee26c088c1ddcbedf35f86d8f453e))
* Remove explicit OpenSSL settings from flake.nix ([1fb0510](https://github.com/shift/garnix-insights/commit/1fb0510a5cab6a226d0f2aad90a8770de7841371))
* Resolve client and cookie scope issues in CLI mode ([b2ff0db](https://github.com/shift/garnix-insights/commit/b2ff0dbdaffcf96f7ee00ae56f6c9b54d9fd4fce))
* Resolve cookie scope in log fetching loop ([a913f63](https://github.com/shift/garnix-insights/commit/a913f6338c298cfebd8542ccf28fd69ebd327379))
* Revert craneLib access and add pkgs.lib.getLib pkgs.openssl to buildInputs ([e77cede](https://github.com/shift/garnix-insights/commit/e77cedec4891d7be8de10f219abe189692a0ea92))
* revert flake.nix paths and commit all changes ([4b6ab0c](https://github.com/shift/garnix-insights/commit/4b6ab0ce39a0ce6d32e1074dcae3d0c663659753))
* Use crane.mkLib pkgs for craneLib in flake.nix ([6789018](https://github.com/shift/garnix-insights/commit/6789018520c506f637eb81f2d28e614f6e091691))
* Use craneLib.buildPackage instead of buildRustPackage ([88f3ab8](https://github.com/shift/garnix-insights/commit/88f3ab816f44d219dfb0edca2b483d44fd70919d))
* Use pkgs.openssl directly for library path in flake.nix ([c2a014a](https://github.com/shift/garnix-insights/commit/c2a014ab03ab0b79585ab0e278eab0c281bf1618))
