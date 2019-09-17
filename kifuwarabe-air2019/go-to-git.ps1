# +
# | UTF-8 with BOM (Powershell)
# |
# | Folder copy.
# +

# +
# | フォルダをゴミ箱に移動する
# +
function Remove-FolderToRecycleBin($dir) {
    if (Test-Path $dir -PathType Container) {
        $fullpath = (Get-Item $dir).FullName

        # +
        # | こんな変なパスぜったい間違う☆（＾～＾）チェックだぜ☆（＾～＾）
        # +
        if ($fullpath -cmatch ".*\\kifuwarabe-air2019\\kifuwarabe-air2019") {
            Write-Host "Remove          | [$fullpath] directory."
            Remove-Item $fullpath -Recurse
        }
        else {
            Write-Host "Ignore          | [$fullpath] directory."
        }
    }
    else {
        Write-Host "Ignore          | [$dir] is not directory or not found."
    }
}


# +
# | Source
# | ------
# +
$sr = "C:\Users\むずでょ\source\repos\kifuwarabe-air2019\kifuwarabe-air2019"
$ds = "C:\Users\むずでょ\Documents\GitHub\kifuwarabe-air2019\kifuwarabe-air2019"

Remove-FolderToRecycleBin($ds)

Write-Host "Copy            | [$sr] --to--> [$ds]."
Copy-Item $sr -destination $ds -recurse


# +
# | Blog
# | ----
# +
$sr = "C:\Users\むずでょ\source\repos\kifuwarabe-air2019\blog"
$ds = "C:\Users\むずでょ\Documents\GitHub\kifuwarabe-air2019\blog"

Remove-FolderToRecycleBin($ds)

Write-Host "Copy            | [$sr] --to--> [$ds]."
Copy-Item $sr -destination $ds -recurse
