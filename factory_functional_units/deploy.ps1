[CmdletBinding()]
$ErrorActionPreference = "Stop"

$context = (Get-Item $PSCommandPath).Directory.Parent
$imageFile = Join-Path $context factory_functional_units\Dockerfile
$imageName = "factory_functional_units"

$match = Get-Content $context\factory_functional_units\Cargo.toml | Select-String -Pattern "^version = `"(?<version>\d+\.\d+\.\d+)`""
if ($match.Matches.Count -eq 0) {
    Write-Error "Could not parse version from Cargo.toml!"
}
$version = $match.Matches[0].Groups["version"].Value

$imageTag = "$version"

docker build -t $imageName -f $imageFile $context
if ($LASTEXITCODE -ne 0) {
    Write-Error "Could not build docker image $imageName!"
}

$imageInRegistry = "docker.pkg.github.com/intelligentagentsystems/rust/$($imageName):$($imageTag)"
$imageInRegistryLatest = "docker.pkg.github.com/intelligentagentsystems/rust/$($imageName):latest"

docker tag $imageName $imageInRegistry
if ($LASTEXITCODE -ne 0) {
    Write-Error "Could not tag docker image $imageName as $imageInRegistry!"
}
docker tag $imageName $imageInRegistryLatest
if ($LASTEXITCODE -ne 0) {
    Write-Error "Could not tag docker image $imageName as $imageInRegistryLatest!"
}

docker push $imageInRegistry
if ($LASTEXITCODE -ne 0) {
    Write-Error "Could not push docker image $imageInRegistry!"
}
docker push $imageInRegistryLatest
if ($LASTEXITCODE -ne 0) {
    Write-Error "Could not push docker image $imageInRegistryLatest!"
}
