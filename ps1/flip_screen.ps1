Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;

public class ScreenRotation
{
    [DllImport("user32.dll")]
    public static extern bool EnumDisplaySettings(string lpszDeviceName, int iModeNum, ref DEVMODE lpDevMode);
    
    [DllImport("user32.dll")]
    public static extern int ChangeDisplaySettings(ref DEVMODE lpDevMode, int dwFlags);

    [StructLayout(LayoutKind.Sequential)]
    public struct DEVMODE
    {
        private const int CCHDEVICENAME = 32;
        private const int CCHFORMNAME = 32;

        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = CCHDEVICENAME)]
        public string dmDeviceName;
        public short dmSpecVersion;
        public short dmDriverVersion;
        public short dmSize;
        public short dmDriverExtra;
        public int dmFields;
        public int dmPositionX;
        public int dmPositionY;
        public int dmDisplayOrientation;
        public int dmDisplayFixedOutput;
        public short dmColor;
        public short dmDuplex;
        public short dmYResolution;
        public short dmTTOption;
        public short dmCollate;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst = CCHFORMNAME)]
        public string dmFormName;
        public short dmLogPixels;
        public int dmBitsPerPel;
        public int dmPelsWidth;
        public int dmPelsHeight;
        public int dmDisplayFlags;
        public int dmDisplayFrequency;
        public int dmICMMethod;
        public int dmICMIntent;
        public int dmMediaType;
        public int dmDitherType;
        public int dmReserved1;
        public int dmReserved2;
        public int dmPanningWidth;
        public int dmPanningHeight;
    }

    public const int ENUM_CURRENT_SETTINGS = -1;
    public const int DISP_CHANGE_SUCCESSFUL = 0;
    public const int DISP_CHANGE_BADMODE = -2;
    public const int DISP_CHANGE_RESTART = 1;
    public const int DMDO_DEFAULT = 0;
    public const int DMDO_90 = 1;
    public const int DMDO_180 = 2;
    public const int DMDO_270 = 3;
}
"@

function Set-ScreenRotation {
    param (
        [Parameter(Mandatory = $true)]
        [ValidateSet(0, 90, 180, 270)]
        [int] $Rotation,
        
        [Parameter(Mandatory = $false)]
        [string] $Monitor = $null
    )

    $dmOrientation = switch ($Rotation) {
        0 { [ScreenRotation]::DMDO_DEFAULT }
        90 { [ScreenRotation]::DMDO_90 }
        180 { [ScreenRotation]::DMDO_180 }
        270 { [ScreenRotation]::DMDO_270 }
    }

    $device = if ($Monitor) { $Monitor } else { $null }

    $dm = New-Object ScreenRotation+DEVMODE
    $dm.dmSize = [System.Runtime.InteropServices.Marshal]::SizeOf($dm)

    [ScreenRotation]::EnumDisplaySettings($device, [ScreenRotation]::ENUM_CURRENT_SETTINGS, [ref]$dm) | Out-Null

    $dm.dmDisplayOrientation = $dmOrientation
    
    $result = [ScreenRotation]::ChangeDisplaySettings([ref]$dm, 0)
    
    switch ($result) {
        [ScreenRotation]::DISP_CHANGE_SUCCESSFUL { Write-Output "Screen orientation changed successfully to $Rotation degrees." }
        [ScreenRotation]::DISP_CHANGE_BADMODE { Write-Error "The requested display mode is not supported." }
        [ScreenRotation]::DISP_CHANGE_RESTART { Write-Warning "You need to restart your computer for the changes to take effect." }
        default { Write-Error "Failed to change screen orientation. Error code: $result" }
    }
}

# Set-ScreenRotation -Rotation 0  # Normal orientation
Set-ScreenRotation -Rotation 90  # Rotate 90 degrees clockwise
# Set-ScreenRotation -Rotation 180  # Rotate 180 degrees (upside down)
# Set-ScreenRotation -Rotation 270  # Rotate 270 degrees (90 degrees counter-clockwise)
