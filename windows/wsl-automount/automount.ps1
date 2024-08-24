$query = "select * from __InstanceCreationEvent within 5 where TargetInstance ISA 'Win32_LogicalDisk'"

$dt =  Get-Date -Format "yyyy/MM/dd HH:mm:ss"
Set-Content ./log.txt "$dt wsl automount start"

Register-WmiEvent -Query $Query -SourceIdentifier USBFlashDriveWSLMount


While ($True) {
  $event = Wait-Event -SourceIdentifier USBFlashDriveWSLMount
  $drivePath = $event.SourceEventArgs.NewEvent.TargetInstance.Name
  $driveLetter = $drivePath.ToLower()[0]
  $dt =  Get-Date -Format "yyyy/MM/dd HH:mm:ss"
  if (((wsl -l -v) -replace "`0" | Select-String -Pattern Run) -ne $null) {
    Add-Content ./log.txt "$dt wsl mounting: $driveLetter $drivePath"
    echo "$dt wsl mounting: $driveLetter $drivePath"
    wsl -u root -e mount -t drvfs $drivePath /mnt/$driveLetter
  } else {
    Add-Content ./log.txt "$dt wsl mount skipped: $driveLetter $drivePath"
    echo "$dt wsl mount skipped: $driveLetter $drivePath"
  }
  Remove-Event USBFlashDriveWSLMount
}

$dt =  Get-Date -Format "yyyy/MM/dd HH:mm:ss"
Add-Content ./log.txt "$dt wsl automount stop"
