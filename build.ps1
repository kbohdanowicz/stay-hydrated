$binOldName = "stay_hydrated.exe"
$binNewName = "Stay Hydrated.exe"

$distPath = "target\release"
$binSrcPath = "$distPath\$binOldName"
$binDestPath = "$distPath\$binNewName"

$resDirSrcPath = "res"
$resDirDestPath = "$distPath\$resDirSrcPath"

cargo build --release

if (Test-Path $binSrcPath) {
    Remove-Item $binDestPath
}
Rename-Item $binSrcPath $binNewName

if (Test-Path $resDirDestPath) {
    Remove-Item -Recurse $resDirDestPath
}
Copy-Item -Recurse $resDirSrcPath $resDirDestPath
