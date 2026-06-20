<?php

return [

    /*
    |--------------------------------------------------------------------------
    | Installer file names
    |--------------------------------------------------------------------------
    |
    | The installer for each platform is served directly from
    | storage/app/downloads/<file> when present. Drop the built installer
    | there (the desktop-release workflow or a deploy step can do this) and
    | the /download/{platform} route streams it instantly from our domain.
    |
    */

    'files' => [
        'windows' => 'ChromaVale-Setup-Windows.exe',
        'mac' => 'ChromaVale-macOS.dmg',
    ],

    /*
    |--------------------------------------------------------------------------
    | Fallback URLs
    |--------------------------------------------------------------------------
    |
    | Used when the installer file is not present on disk yet. Point these at
    | a direct release asset for an instant download, or leave the default
    | releases page.
    |
    */

    'fallback' => [
        'windows' => env('DOWNLOAD_URL_WINDOWS', 'https://github.com/tanerozg/chromavale/releases/latest'),
        'mac' => env('DOWNLOAD_URL_MAC', 'https://github.com/tanerozg/chromavale/releases/latest'),
    ],

];
