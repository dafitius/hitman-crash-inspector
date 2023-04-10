# Hitman Crash Inspector
<img src="https://github.com/dafitius/hitman-crash-inspector/blob/main/assets/icon.ico" width="150" height="150">


Hitman Crash Inspector is a terminal-based tool for decrypting and inspecting the `crash_metrics.dat` files from [HITMAN World of Assassination](https://ioi.dk/hitman). The tool provides detailed information on the crash described inside the crash metrics file.

## Usage
To use the GUI, simply download the latest release from the [releases]() page and extract the contents of the zip file. You can then use the program by running the `hitman-crash-inspector.exe` file. The tool will automatically open a terminal, and you can select a `crash_metrics.dat` file manually. 

### command-line arguments
You can use the tool in a command-line by passing arguments to the `hitman-crash-inspector.exe` file. The following arguments are available:

```
Usage: hitman-crash-inspector.exe [--tick-rate <tick-rate>] [--enhanced-graphics <enhanced-graphics>] [--metrics-path <metrics-path>]

Options:
  --tick-rate       
        time in ms between two ticks (default: 150)
  --enhanced-graphics 
        whether unicode symbols are used to improve the overall look of the app (default: true)
  --metrics-path   
        path to the metrics file
  --help            
        display usage information
```
For example, to open a crash_metrics.dat file located at `C:\Users\agent_47\AppData\Roaming\IO Interactive\HITMAN3\crash_metrics.dat` and enable live update, you can use the following command:

```cmd
hitman-crash-inspector.exe --metrics-path "C:\Users\agent_47\AppData\Roaming\IO Interactive\HITMAN3\crash_metrics.dat"
```

### Controls
You can control the tool using:
```
1-8: Switch to the corresponding tab.
s: Save the current data to a `.json` file.
i: Import a file to be analyzed.
l: Toggle live update. When enabled, the tool will automatically refresh its values if there are changes to the imported file.
q: Quit the tool.
```
