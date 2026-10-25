<?php

// Fixture generation only, never loaded by Pun or the analysed project. PHP 8.5.12:
// php -d phar.readonly=0 create-archives.php
// PHP produces the valid manifests/signatures; malformed controls alter explicitly identified format fields.
$dir = __DIR__ . '/archives';
if (!is_dir($dir)) {
    mkdir($dir);
}
foreach (glob($dir . '/*') as $file) {
    if (is_file($file)) {
        unlink($file);
    }
}
foreach (['native-md5' => Phar::MD5, 'native-sha1' => Phar::SHA1, 'native' => Phar::SHA256, 'native-sha512' => Phar::SHA512] as $name => $signature) {
    $phar = new Phar("$dir/$name.phar");
    $phar['ok.php'] = '<?php throw new Exception("must never run");';
    $phar['dir/child.php'] = '<?php';
    $phar['percent%20name.php'] = '<?php';
    $phar->addEmptyDir('empty-dir');
    $phar->setStub('<?php __HALT_COMPILER(); ?>');
    $phar->setSignatureAlgorithm($signature);
    if ($name === 'native') {
        $phar->convertToExecutable(Phar::ZIP);
        $phar->convertToExecutable(Phar::TAR);
        $phar->compress(Phar::GZ);
    }
    unset($phar);
}
$phar = new Phar("$dir/entry-gzip.phar");
$phar['ok.php'] = '<?php';
$phar->compressFiles(Phar::GZ);
unset($phar);
foreach (['data.zip' => Phar::ZIP, 'data.tar' => Phar::TAR] as $name => $format) {
    $phar = new PharData("$dir/$name", 0, null, $format);
    $phar['ok.php'] = '<?php';
    $phar['dir/child.php'] = '<?php';
    $phar->addEmptyDir('empty-dir');
    if ($format === Phar::TAR) {
        $phar->compress(Phar::GZ);
    }
    unset($phar);
}
$zip = new ZipArchive();
$zip->open("$dir/data.zip");
$zip->addFromString('compressed.php', str_repeat('<?php', 100));
$zip->setCompressionName('compressed.php', ZipArchive::CM_DEFLATE);
$zip->addFromString('back\\', '<?php');
$zip->close();

$native = file_get_contents("$dir/native.phar");
// ext/phar/phar.c decrements its restart counter before dispatch: two gzip layers work, three do not.
$nested = $native;
for ($layers = 1; $layers <= 4; $layers++) {
    $nested = gzencode($nested);
    if ($layers >= 2) {
        file_put_contents("$dir/nested-$layers.phar.gz", $nested);
    }
}
copy("$dir/native.phar", "$dir/archive.bin");
copy("$dir/native.phar", "$dir/noextension");
// Change the stub while retaining the manifest and ask PHP's hash implementation to re-sign it.
function signed(string $body): string {
    return $body . hash('sha256', $body, true) . pack('V', Phar::SHA256) . 'GBMB';
}
$body = substr($native, 0, -40);
$manifest = strpos($body, '__HALT_COMPILER();') + strlen("__HALT_COMPILER(); ?>\r\n");
$tail = substr($body, $manifest);
file_put_contents("$dir/stub-bare.phar", signed('<?php __HALT_COMPILER();' . $tail));
file_put_contents("$dir/stub-lf.phar", signed("<?php __halt_compiler(); ?>\n" . $tail));
file_put_contents("$dir/stub-bad-cr.phar", signed("<?php __HALT_COMPILER(); ?>\rx" . $tail));
file_put_contents("$dir/bad-signature.phar", substr_replace($native, $native[-40] ^ "\x80", -40, 1));
file_put_contents("$dir/bad-trailer.phar", substr_replace($native, 'FAIL', -4));
file_put_contents("$dir/bad-algorithm.phar", substr_replace($native, pack('V', 99), -8, 4));
file_put_contents("$dir/oversized.phar", substr_replace($native, pack('V', 104857601), $manifest, 4));
file_put_contents("$dir/truncated.phar", substr($native, 0, $manifest + 10));
file_put_contents("$dir/zero-count.phar", signed(substr_replace($body, pack('V', 0), $manifest + 4, 4)));
file_put_contents("$dir/old-version.phar", signed(substr_replace($body, "\x00\x00", $manifest + 8, 2)));
file_put_contents("$dir/unsigned.phar", signed(substr_replace($body, pack('V', 0), $manifest + 10, 4)));
// First entry starts after: length(4), count(4), version(2), flags(4), alias(4), metadata(4).
$entry = $manifest + 22;
$nameSize = unpack('V', substr($body, $entry, 4))[1];
file_put_contents("$dir/empty-name.phar", signed(substr_replace($body, pack('V', 0), $entry, 4)));
file_put_contents("$dir/bad-size.phar", signed(substr_replace($body, pack('V', 12345), $entry + 4 + $nameSize + 8, 4)));
file_put_contents("$dir/bzip-entry.phar", signed(substr_replace($body, pack('V', 0x2000 | 0644), $entry + 4 + $nameSize + 16, 4)));
file_put_contents("$dir/bad.zip", 'PK' . "\x03\x04" . 'broken');
file_put_contents("$dir/bad.tar", str_repeat('broken', 100));
file_put_contents("$dir/bad.gz", "\x1f\x8b\x08" . 'broken');
foreach (['zip', 'tar'] as $format) {
    $name = "$dir/native.phar.$format";
    $bytes = file_get_contents($name);
    $offset = $format === 'zip' ? 10 : strpos($bytes, '.phar/signature.bin') + 512 + 8;
    file_put_contents("$dir/bad-signature.$format", substr_replace($bytes, $bytes[$offset] ^ "\x80", $offset, 1));
}
// Executable archives without signatures are not accepted even though their indexes are valid.
$zip = new ZipArchive();
$zip->open("$dir/no-signature.zip", ZipArchive::CREATE);
$zip->addFromString('ok.php', '<?php');
$zip->addFromString('.phar/stub.php', '<?php __HALT_COMPILER();');
$zip->close();
$tar = file_get_contents("$dir/native.phar.tar");
$signature = strpos($tar, '.phar/signature.bin');
file_put_contents("$dir/no-signature.tar", substr($tar, 0, $signature) . str_repeat("\0", 1024));
file_put_contents("$dir/after-signature.tar", substr($tar, 0, $signature + 1024) . substr($tar, 0, 1024) . str_repeat("\0", 1024));
$zip = new ZipArchive();
$zip->open("$dir/unknown-signature.zip", ZipArchive::CREATE);
$zip->addFromString('ok.php', '<?php');
$zip->addFromString('.phar/signature.bin', pack('VV', 99, 32) . str_repeat('x', 32));
$zip->close();
$bytes = file_get_contents("$dir/data.zip");
$bytes = substr_replace($bytes, pack('v', 99), 8, 2);
$bytes = substr_replace($bytes, pack('v', 99), strpos($bytes, "PK\x01\x02") + 10, 2);
file_put_contents("$dir/bad-compression.zip", $bytes);
// Derive ordinary USTAR link records from PHP's valid data archive. PHP's own is_file() is the oracle for
// the asymmetric hard-link target validation and dangling-symlink stat behavior.
$data = substr(file_get_contents("$dir/data.tar"), 0, 1024);
file_put_contents("$dir/truncated.tar", substr($data, 0, 514));
function linkHeader(string $data, string $name, string $type, string $target): string {
    $header = substr($data, 0, 512);
    $header = substr_replace($header, str_pad($name, 100, "\0"), 0, 100);
    $header = substr_replace($header, "00000000000\0", 124, 12);
    $header[156] = $type;
    $header = substr_replace($header, str_pad($target, 100, "\0"), 157, 100);
    $header = substr_replace($header, str_repeat(' ', 8), 148, 8);
    return substr_replace($header, sprintf("%06o\0 ", array_sum(unpack('C*', $header))), 148, 8);
}
file_put_contents("$dir/links.tar", $data . linkHeader($data, 'hard.php', '1', 'ok.php') . linkHeader($data, 'dangling.php', '2', 'missing.php') . str_repeat("\0", 1024));
file_put_contents("$dir/bad-links.tar", $data . linkHeader($data, 'hard.php', '1', 'missing.php') . str_repeat("\0", 1024));
