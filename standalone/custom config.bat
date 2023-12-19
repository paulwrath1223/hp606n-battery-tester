@echo off
echo ----------------------------------
set /p ip_addr= "Enter IP address of GPIB adapter: "
echo ----------------------------------
echo %ip_addr%
echo ----------------------------------

echo ----------------------------------
set /p gpib_addr= "Enter GPIB address of HP606n device: "
echo ----------------------------------
echo %gpib_addr%
echo ----------------------------------

echo ----------------------------------
set /p discharge_current= "Enter discharge current: "
echo ----------------------------------
echo %discharge_current%
echo ----------------------------------

echo ----------------------------------
set /p file_name= "Enter name of output file excluding `.csv`: "
echo ----------------------------------
echo %file_name%
echo ----------------------------------

echo ----------------------------------
set /p poll_rate= "Enter polling rate (queries per minute): "
echo ----------------------------------
echo %poll_rate%
echo ----------------------------------

echo ----------------------------------
set /p cutoff_voltage= "Enter cutoff voltage (voltage to stop discharging once reached): "
echo ----------------------------------
echo %cutoff_voltage%
echo ----------------------------------

echo ----------------------------------
set /p current_range= "Enter current range (see your device's manual to find what range best fits your desired current draw): "
echo ----------------------------------
echo %current_range%
echo ----------------------------------

hp606n-battery-tester.exe --ip-string "%ip_addr%" --gpib-addr %gpib_addr% --discharge-current %discharge_current% --output-file %file_name%.csv --polling-rate %poll_rate% --cutoff-voltage %cutoff_voltage% --current-range %current_range%

pause