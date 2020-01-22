[CmdletBinding()]
Param (
    [Parameter(Mandatory=$true)] [string] $Version
)
$ErrorActionPreference = "Stop"

$context = (Get-Item $PSCommandPath).Directory.Parent
$imageFile = Join-Path $context OrderClient\Dockerfile
$imageName = "order_client"

$imageTag = "$Version"

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
