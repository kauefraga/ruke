Invoke-WebRequest 'https://github.com/kauefraga/ruke/releases/latest/download/ruke-x86_64-pc-windows-msvc.zip' -OutFile 'ruke-x86_64-pc-windows-msvc.zip'

Expand-Archive -Force -Path 'ruke-x86_64-pc-windows-msvc.zip' -DestinationPath '.'

Remove-Item 'ruke-x86_64-pc-windows-msvc.zip'

Write-Host 'Reminder: add ruke in your PATH or move it to a local where PATH is already set'
