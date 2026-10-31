<?php
namespace App;
final class T
{
    public function a1() { try { return [1]; } catch (\Exception $e) { return [2]; } }
    public function a2() { try { return [1]; } catch (\Exception $e) { return ['a']; } }
    public function a3() { try { return [1]; } catch (\Exception $e) { return [1, 2]; } }
    public function a4() { try { return ['a' => 1]; } catch (\Exception $e) { return ['b' => 1]; } }
    public function a5() { try { return []; } catch (\Exception $e) { return []; } }
    public function a6() { try { return [1]; } catch (\Exception $e) { return []; } }
    public function a7() { try { return [1]; } catch (\Exception $e) { return [true]; } }
    public function a8() { try { return [1, 'a']; } catch (\Exception $e) { return [2, 'b']; } }
    public function a9(int $i) { try { return [$i]; } catch (\Exception $e) { return [$i, $i]; } }
    public function a10() { try { return 1; } catch (\Exception $e) { return 2.5; } }
    public function a11() { try { return 1; } catch (\Exception $e) { return null; } }
    public function a12() { try { return 'a'; } catch (\Exception $e) { return A::class; } }
    public function a13(string $s) { try { return 'a'; } catch (\Exception $e) { return $s; } }
    public function a14(int $i) { try { return 5; } catch (\Exception $e) { return $i; } }
    public function a15() { try { return true; } catch (\Exception $e) { return 1 > 2; } }
    public function a16(bool $b) { try { return $b; } catch (\Exception $e) { return false; } }
    public function a17() { try { return 1.5; } catch (\Exception $e) { return 2; } }
    public function a18() { try { return $this; } catch (\Exception $e) { return new self; } }
    public function a19() { try { return static::class; } catch (\Exception $e) { return 'a'; } }
    public function a20() { try { return 1; } catch (\Exception $e) { return 2; } catch (\Error $e) { return 'a'; } }
    public function a21() { try { return PHP_INT_MAX; } catch (\Exception $e) { return 2; } }
    public function a22() { try { return -1; } catch (\Exception $e) { return 2; } }
    public function a23() { try { return 1 ? 2 : 3; } catch (\Exception $e) { return 2; } }
}
-----
<?php
namespace App;
final class T
{
    public function a1(): array { try { return [1]; } catch (\Exception $e) { return [2]; } }
    public function a2(): array { try { return [1]; } catch (\Exception $e) { return ['a']; } }
    public function a3(): array { try { return [1]; } catch (\Exception $e) { return [1, 2]; } }
    public function a4(): array { try { return ['a' => 1]; } catch (\Exception $e) { return ['b' => 1]; } }
    public function a5(): array { try { return []; } catch (\Exception $e) { return []; } }
    public function a6(): array { try { return [1]; } catch (\Exception $e) { return []; } }
    public function a7(): array { try { return [1]; } catch (\Exception $e) { return [true]; } }
    public function a8(): array { try { return [1, 'a']; } catch (\Exception $e) { return [2, 'b']; } }
    public function a9(int $i): array { try { return [$i]; } catch (\Exception $e) { return [$i, $i]; } }
    public function a10() { try { return 1; } catch (\Exception $e) { return 2.5; } }
    public function a11() { try { return 1; } catch (\Exception $e) { return null; } }
    public function a12(): string { try { return 'a'; } catch (\Exception $e) { return A::class; } }
    public function a13(string $s) { try { return 'a'; } catch (\Exception $e) { return $s; } }
    public function a14(int $i) { try { return 5; } catch (\Exception $e) { return $i; } }
    public function a15(): true { try { return true; } catch (\Exception $e) { return 1 > 2; } }
    public function a16(bool $b): bool { try { return $b; } catch (\Exception $e) { return false; } }
    public function a17() { try { return 1.5; } catch (\Exception $e) { return 2; } }
    public function a18(): static { try { return $this; } catch (\Exception $e) { return new self; } }
    public function a19() { try { return static::class; } catch (\Exception $e) { return 'a'; } }
    public function a20() { try { return 1; } catch (\Exception $e) { return 2; } catch (\Error $e) { return 'a'; } }
    public function a21() { try { return PHP_INT_MAX; } catch (\Exception $e) { return 2; } }
    public function a22(): int { try { return -1; } catch (\Exception $e) { return 2; } }
    public function a23(): int { try { return 1 ? 2 : 3; } catch (\Exception $e) { return 2; } }
}
