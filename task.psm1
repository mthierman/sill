function Export-Task
{
    [ordered]@{
        test     = {
            Get-ChildItem
        }
        metadata = {
            cargo metadata --no-deps --format-version=1 > metadata.json
        }
    }
}
