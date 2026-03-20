$dest = "$env:USERPROFILE\.count"
New-Item -ItemType Directory -Force -Path $dest
Invoke-WebRequest -Uri "https://github.com/WhyNaught/count/releases/download/v1.0.0/count-windows.exe" -OutFile "$dest\count.exe"
[Environment]::SetEnvironmentVariable("Path", $env:Path + ";$dest", "User")