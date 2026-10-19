<?php

namespace App;

const TOP = 5;

interface I
{
}

final class K
{
    public const A = 'a';
    public const B = 1;
    public const C = 1.5;
    public const D = [1];
    public const E = true;
    public const F = null;
    public const G = K::A;

    public function a() { return self::A; }
    public function b() { return self::B; }
    public function c() { return self::C; }
    public function d() { return self::D; }
    public function e() { return self::E; }
    public function f() { return self::F; }
    public function g() { return self::G; }
    public function h() { return TOP; }
    public function i() { return PHP_EOL; }
    public function j() { return true; }
    public function k() { return null; }
    public function l($x) { if ($x) { return true; } return false; }
    public function m($x) { if ($x) { return self::A; } return self::B; }
    public function n($x) { if ($x) { return self::A; } return null; }
    public function o() { return K::class; }
    public function p() { return PHP_INT_MAX; }
    public function q($x) { if ($x) { return false; } return null; }
    public function r($x) { if ($x) { return self::A; } return false; }
    public function s() { return false; }
    public function t($x) { if ($x) { return self::A; } return true; }
    public function u() { return \M_PI; }
    public function v() { return UNKNOWN; }
    public function w() { return I::class; }
}
-----
<?php

namespace App;

const TOP = 5;

interface I
{
}

final class K
{
    public const A = 'a';
    public const B = 1;
    public const C = 1.5;
    public const D = [1];
    public const E = true;
    public const F = null;
    public const G = K::A;

    public function a(): string { return self::A; }
    public function b(): int { return self::B; }
    public function c(): float { return self::C; }
    public function d(): array { return self::D; }
    public function e(): bool { return self::E; }
    public function f() { return self::F; }
    public function g(): string { return self::G; }
    public function h(): int { return TOP; }
    public function i(): string { return PHP_EOL; }
    public function j(): bool { return true; }
    public function k() { return null; }
    public function l($x): bool { if ($x) { return true; } return false; }
    public function m($x) { if ($x) { return self::A; } return self::B; }
    public function n($x): ?string { if ($x) { return self::A; } return null; }
    public function o(): string { return K::class; }
    public function p(): int { return PHP_INT_MAX; }
    public function q($x): ?bool { if ($x) { return false; } return null; }
    public function r($x) { if ($x) { return self::A; } return false; }
    public function s(): bool { return false; }
    public function t($x) { if ($x) { return self::A; } return true; }
    public function u(): float { return \M_PI; }
    public function v() { return UNKNOWN; }
    public function w(): string { return I::class; }
}
