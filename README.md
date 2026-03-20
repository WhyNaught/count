# Count
## A lightweight CLI tool that counts how many lines of code you have

Count is a lightweight CLI tool to count the number of lines of code in a given directory or file! 
Made in Rust. 

### Usage
```bash
count <file/directory path> [--ignore filepath_to_ignore] [--suffix suffix_to_filter_by] [--help]
```

### Installation
Ensure you have [Curl](https://curl.se/download.html) installed. 

#### MacOS and Linux
```bash
curl -sSL https://raw.githubusercontent.com/WhyNaught/count/main/count/build/install.sh | bash
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