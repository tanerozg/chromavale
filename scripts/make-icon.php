<?php

/**
 * Generates public/apple-touch-icon.png - the ChromaVale lens mark:
 * dark background, white ring, prism-gradient core.
 */

$size = 180;
$im = imagecreatetruecolor($size, $size);
imagealphablending($im, true);
imagesavealpha($im, true);

// Background (iOS applies its own corner mask).
$bg = imagecolorallocate($im, 0x16, 0x16, 0x1A);
imagefilledrectangle($im, 0, 0, $size, $size, $bg);

$cx = $size / 2;
$cy = $size / 2;

// White ring: white disc, then punch the centre back to background.
$white = imagecolorallocate($im, 0xFF, 0xFF, 0xFF);
$ringOuter = 110; // diameter
$ringThickness = 13;
imagefilledellipse($im, $cx, $cy, $ringOuter, $ringOuter, $white);
imagefilledellipse($im, $cx, $cy, $ringOuter - 2 * $ringThickness, $ringOuter - 2 * $ringThickness, $bg);

// Prism core: horizontal spectrum clipped to a circle.
$stops = [
    [0xFF, 0x7A, 0x6B],
    [0xFF, 0xC2, 0x4B],
    [0x5B, 0xD6, 0xA0],
    [0x4D, 0xA6, 0xFF],
    [0x9B, 0x6B, 0xE5],
];
$r = 27; // core radius

$lerp = function ($a, $b, $t) {
    return (int) round($a + ($b - $a) * $t);
};

for ($x = -$r; $x <= $r; $x++) {
    $t = ($x + $r) / (2 * $r); // 0..1 across the spectrum
    $pos = $t * (count($stops) - 1);
    $i = (int) floor($pos);
    if ($i >= count($stops) - 1) {
        $i = count($stops) - 2;
    }
    $f = $pos - $i;
    $c0 = $stops[$i];
    $c1 = $stops[$i + 1];
    $color = imagecolorallocate(
        $im,
        $lerp($c0[0], $c1[0], $f),
        $lerp($c0[1], $c1[1], $f),
        $lerp($c0[2], $c1[2], $f),
    );
    $h = (int) round(sqrt(max(0, $r * $r - $x * $x)));
    imageline($im, $cx + $x, $cy - $h, $cx + $x, $cy + $h, $color);
}

imagepng($im, __DIR__ . '/../public/apple-touch-icon.png');
imagedestroy($im);

echo "apple-touch-icon.png written\n";
