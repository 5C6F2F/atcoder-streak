param (
    $taskname,
    $binary_path,
    $description
)

if (!([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole("Administrators")) {
    Start-Process pwsh "-File $PSCommandPath $taskname $binary_path $description $args" -Verb RunAs
    Read-Host
    exit
}

$action = New-ScheduledTaskAction -Execute $binary_path
$triggers = @()
foreach ($time in $args) {
    $trigger = New-ScheduledTaskTrigger -Daily -At $time
    $triggers += $trigger
}
$settings = New-ScheduledTaskSettingsSet
$task = New-ScheduledTask -Action $action -Trigger $triggers -Settings $settings -Description $description
Register-ScheduledTask -TaskName $taskname -InputObject $task -Force

Write-Host
Write-Host
Write-Host Enterで終了...
Read-Host
