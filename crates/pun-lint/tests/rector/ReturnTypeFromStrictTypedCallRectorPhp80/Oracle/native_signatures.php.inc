<?php

namespace App;

final class NativeSignatures
{
    public function includePath()
    {
        return get_include_path();
    }

    public function curl()
    {
        return curl_init();
    }

    public function temporary()
    {
        return tmpfile();
    }

    public function absolute(int|float $x)
    {
        return abs($x);
    }

    public function encoded(string $s)
    {
        return base64_encode($s);
    }

    public function values(array $a)
    {
        return array_values($a);
    }

    public function replaced(string $s)
    {
        return preg_replace('/a/', 'b', $s);
    }

    public function replacedMany(array|string $s)
    {
        return preg_replace_callback('/a/', fn ($m) => 'b', $s);
    }

    public function printed($x, bool $return)
    {
        return print_r($x, $return);
    }

    public function exported($x, bool $return)
    {
        return var_export($x, $return);
    }

    public function headers(string $url, bool $associative)
    {
        return get_headers($url, $associative);
    }

    public function highlighted(string $file, bool $return)
    {
        return highlight_file($file, $return);
    }

    public function isA($x, bool $allow)
    {
        return is_a($x, 'App\NativeSignatures', $allow);
    }
}
-----
<?php

namespace App;

final class NativeSignatures
{
    public function includePath(): string|false
    {
        return get_include_path();
    }

    public function curl(): \CurlHandle|false
    {
        return curl_init();
    }

    public function temporary()
    {
        return tmpfile();
    }

    public function absolute(int|float $x): float|int
    {
        return abs($x);
    }

    public function encoded(string $s): string
    {
        return base64_encode($s);
    }

    public function values(array $a)
    {
        return array_values($a);
    }

    public function replaced(string $s): ?string
    {
        return preg_replace('/a/', 'b', $s);
    }

    public function replacedMany(array|string $s): string|array|null
    {
        return preg_replace_callback('/a/', fn ($m) => 'b', $s);
    }

    public function printed($x, bool $return): string|bool
    {
        return print_r($x, $return);
    }

    public function exported($x, bool $return): ?string
    {
        return var_export($x, $return);
    }

    public function headers(string $url, bool $associative): array|false
    {
        return get_headers($url, $associative);
    }

    public function highlighted(string $file, bool $return): string|bool
    {
        return highlight_file($file, $return);
    }

    public function isA($x, bool $allow): bool
    {
        return is_a($x, 'App\NativeSignatures', $allow);
    }
}
