param([switch]$Check)

function has($command) {
    return !!(Get-Command -ErrorAction SilentlyContinue $command)
}

# Check dependencies
if(!(has rustup)) {
    throw "Rustup must be installed manaully. Go to https://rustup.rs/ and use rustup to install it."
}

$activeToolchain = rustup show active-toolchain
if(!$activeToolchain.StartsWith("nightly-")) {
    if($Check) {
        throw "The nightly rust toolchain must be active. Use 'rustup update nightly' to install it and 'rustup default nightly' to enable it. Or run scripts/install-prereqs.ps1."
    }
    else {
        rustup default nightly
        if($LASTEXITCODE -ne 0) {
            rustup update nightly
            if($LASTEXITCODE -ne 0) {
                throw "Failed to install nightly rust"
            }
            rustup default nightly
            if($LASTEXITCODE -ne 0) {
                throw "Failed to enable nightly rust"
            }
        }
    }
}

if(!(has cargo-xbuild)) {
    if($Check) {
        throw "cargo-xbuild must be installed. Use 'cargo install cargo-xbuild' to install it. Or run scripts/install-prereqs.ps1."
    } else {
        cargo install cargo-xbuild
    }
}

if(!(has bootimage)) {
    if($Check) {
        throw "bootimage must be installed. Use 'cargo install bootimage' to install it. Or run scripts/install-prereqs.ps1."
    } else {
        cargo install bootimage --version "^0.5.0"
    }
}