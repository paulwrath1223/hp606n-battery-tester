email me if you have questions, this software has very few people who will likely ever need this, and I won't write a README if no one needs it.
my email is paul@fornage.net

However people should know that the exe needs launch arguments to work, or else it will just flash. run the program using the bat scripts or by making a shortcut to the exe and putting arguments in there.

##Arguments
```
Usage: hp606n-battery-tester.exe [OPTIONS] --ip-string <IP_STRING> --gpib-addr <GPIB_ADDR> --discharge-current <DISCHARGE_CURRENT> --output-file <OUTPUT_FILE> --polling-rate <POLLING_RATE>

Options:
  -i, --ip-string <IP_STRING>
          IP address of the controller
  -g, --gpib-addr <GPIB_ADDR>
          the address to find the load device (HP606n) on the GPIB bus
  -d, --discharge-current <DISCHARGE_CURRENT>
          the current to draw from the battery in Amps
  -c, --current-range <CURRENT_RANGE>
          the range for the current sensor on the machine. this differs on different machines, so check which range best matches your discharge current. This field is the index of the range, so for the 6063B which has ranges [0, 1A] and [0, 10A], you would enter `0` for discharge currents less than 1A, a
nd `1` otherwise [default: 1]
  -v, --cutoff-voltage <CUTOFF_VOLTAGE>
          the voltage to stop discharging the battery [default: 0]
  -p, --output-file <OUTPUT_FILE>
          path and name for the output file
  -r, --polling-rate <POLLING_RATE>
          number of times to check the voltage per minute. This affects cutoff voltage checks if set too low
  -h, --help
          Print help
```
