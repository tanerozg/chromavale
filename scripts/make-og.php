<?php

/**
 * Generates public/og-image.png — a 1200x630 social share card for
 * ChromaVale: dark background, lens mark, wordmark, tagline, spectrum bar.
 */

$W = 1200;
$H = 630;
$im = imagecreatetruecolor($W, $H);
imagealphablending($im, true);
imagesavealpha($im, true);

$bg = imagecolorallocate($im, 0x13, 0x13, 0x17);

// Subtle vertical gradient for depth (lighter at the top), no decorative blob.
for ($y = 0; $y < $H; $y++) {
    $t = $y / $H;
    $shade = imagecolorallocate(
        $im,
        (int) round(0x1A - $t * 0x09),
        (int) round(0x1A - $t * 0x09),
        (int) round(0x20 - $t * 0x0C),
    );
    imageline($im, 0, $y, $W, $y, $shade);
}

$padX = 96;
$top = 150;

// Lens mark.
$cx = $padX + 48;
$cy = $top + 6;
$white = imagecolorallocate($im, 0xFF, 0xFF, 0xFF);
// Punch the ring centre back to the local gradient colour at this height.
$tc = $cy / $H;
$localBg = imagecolorallocate(
    $im,
    (int) round(0x1A - $tc * 0x09),
    (int) round(0x1A - $tc * 0x09),
    (int) round(0x20 - $tc * 0x0C),
);
imagefilledellipse($im, $cx, $cy, 96, 96, $white);
imagefilledellipse($im, $cx, $cy, 72, 72, $localBg);

$stops = [
    [0xFF, 0x7A, 0x6B],
    [0xFF, 0xC2, 0x4B],
    [0x5B, 0xD6, 0xA0],
    [0x4D, 0xA6, 0xFF],
    [0x9B, 0x6B, 0xE5],
];
$lerp = fn ($a, $b, $t) => (int) round($a + ($b - $a) * $t);
$spectrum = function ($t) use ($stops, $lerp) {
    $pos = $t * (count($stops) - 1);
    $i = min((int) floor($pos), count($stops) - 2);
    $f = $pos - $i;
    return [
        $lerp($stops[$i][0], $stops[$i + 1][0], $f),
        $lerp($stops[$i][1], $stops[$i + 1][1], $f),
        $lerp($stops[$i][2], $stops[$i + 1][2], $f),
    ];
};

$r = 23;
for ($x = -$r; $x <= $r; $x++) {
    [$rr, $gg, $bb] = $spectrum(($x + $r) / (2 * $r));
    $color = imagecolorallocate($im, $rr, $gg, $bb);
    $h = (int) round(sqrt(max(0, $r * $r - $x * $x)));
    imageline($im, $cx + $x, $cy - $h, $cx + $x, $cy + $h, $color);
}

// Fonts (Windows system fonts).
$fontSemibold = 'C:/Windows/Fonts/segoeuisb.ttf';
$fontRegular = 'C:/Windows/Fonts/segoeui.ttf';
if (! file_exists($fontSemibold)) {
    $fontSemibold = 'C:/Windows/Fonts/segoeuib.ttf';
}

// Wordmark beside the lens.
imagettftext($im, 40, 0, $cx + 78, $cy + 16, $white, $fontSemibold, 'ChromaVale');

// Headline.
$head = imagecolorallocate($im, 0xFA, 0xFA, 0xFB);
imagettftext($im, 62, 0, $padX, 330, $head, $fontSemibold, 'Your screen,');
imagettftext($im, 62, 0, $padX, 410, $head, $fontSemibold, 'tuned to your eyes.');

// Tagline, shrunk to fit within the content width.
$muted = imagecolorallocate($im, 0xA1, 0xA1, 0xAA);
$tagline = 'Screen color control and color-blind correction. Mac and Windows.';
$maxWidth = $W - 2 * $padX;
$tagSize = 27;
do {
    $box = imagettfbbox($tagSize, 0, $fontRegular, $tagline);
    $textWidth = abs($box[2] - $box[0]);
    if ($textWidth <= $maxWidth) {
        break;
    }
    $tagSize -= 1;
} while ($tagSize > 16);
imagettftext($im, $tagSize, 0, $padX, 470, $muted, $fontRegular, $tagline);

// Spectrum bar near the bottom.
$barY = 545;
$barH = 12;
$barX0 = $padX;
$barX1 = $W - $padX;
for ($x = $barX0; $x <= $barX1; $x++) {
    [$rr, $gg, $bb] = $spectrum(($x - $barX0) / ($barX1 - $barX0));
    $color = imagecolorallocate($im, $rr, $gg, $bb);
    imageline($im, $x, $barY, $x, $barY + $barH, $color);
}

imagepng($im, __DIR__ . '/../public/og-image.png');
imagedestroy($im);

echo "og-image.png written\n";
