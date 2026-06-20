<?php

/**
 * Generates public/icon-source.png (1024x1024): the ChromaVale lens mark on a
 * dark rounded tile, used as the source for `tauri icon`.
 */

$size = 1024;
$im = imagecreatetruecolor($size, $size);
imagesavealpha($im, true);
imagealphablending($im, false);
$transparent = imagecolorallocatealpha($im, 0, 0, 0, 127);
imagefilledrectangle($im, 0, 0, $size, $size, $transparent);
imagealphablending($im, true);

// Rounded dark tile.
$bg = imagecolorallocate($im, 0x16, 0x16, 0x1A);
$radius = 220;
$pad = 8;
$x0 = $pad;
$y0 = $pad;
$x1 = $size - $pad;
$y1 = $size - $pad;
imagefilledrectangle($im, $x0 + $radius, $y0, $x1 - $radius, $y1, $bg);
imagefilledrectangle($im, $x0, $y0 + $radius, $x1, $y1 - $radius, $bg);
imagefilledarc($im, $x0 + $radius, $y0 + $radius, $radius * 2, $radius * 2, 180, 270, $bg, IMG_ARC_PIE);
imagefilledarc($im, $x1 - $radius, $y0 + $radius, $radius * 2, $radius * 2, 270, 360, $bg, IMG_ARC_PIE);
imagefilledarc($im, $x0 + $radius, $y1 - $radius, $radius * 2, $radius * 2, 90, 180, $bg, IMG_ARC_PIE);
imagefilledarc($im, $x1 - $radius, $y1 - $radius, $radius * 2, $radius * 2, 0, 90, $bg, IMG_ARC_PIE);

$cx = $size / 2;
$cy = $size / 2;

// White ring.
$white = imagecolorallocate($im, 0xFF, 0xFF, 0xFF);
$ringOuter = 560;
$ringThickness = 64;
imagesetthickness($im, 1);
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
$lerp = fn ($a, $b, $t) => (int) round($a + ($b - $a) * $t);
$r = 150;
for ($x = -$r; $x <= $r; $x++) {
    $t = ($x + $r) / (2 * $r);
    $pos = $t * (count($stops) - 1);
    $i = min((int) floor($pos), count($stops) - 2);
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
    imageline($im, (int) ($cx + $x), (int) ($cy - $h), (int) ($cx + $x), (int) ($cy + $h), $color);
}

imagepng($im, __DIR__ . '/../public/icon-source.png');
imagedestroy($im);

echo "icon-source.png written\n";
