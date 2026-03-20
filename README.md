# Count
## A lightweight CLI tool that counts how many lines of code you have

![Version](https://img.shields.io/badge/version-1.0.0-blue)
![License](https://img.shields.io/github/license/WhyNaught/count)
![Downloads](https://img.shields.io/github/downloads/WhyNaught/count/total)

Count is a lightweight CLI tool to count the number of lines of code in a given directory or file! 
Made in Rust. 

### Usage
```bash
count <file/directory path> [--ignore filepath_to_ignore] [--suffix suffix_to_filter_by] [--help] [--num-only]
```

### Installation
Ensure you have [Curl](https://curl.se/download.html) installed. 

#### MacOS and Linux
```bash
curl -sSL https://raw.githubusercontent.com/WhyNaught/count/main/count/build/install.sh | bash
```

Afterwards, run: 
```bash
chmod +x /usr/local/bin/count
```

#### Windows 
```powershell
curl -sSL https://raw.githubusercontent.com/WhyNaught/count/main/count/build/install.ps1 | powershell -Command -
```

### Verify Installation
Run the following command to verify if the installation was successful:
```bash
count --help
```

If you receive instructions on how to use the command, the install was successful!

### Common Issues
If the 'count' command doesn't work, add it to your system's file path, then restart your terminal and try again. 