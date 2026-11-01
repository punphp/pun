<?php
namespace App;
class A {} class B extends A {}
final class T
{
    public function m1() { try { return 1; } catch (\Exception $e) { return 'a'; } }
    public function m2() { try { return 'x'; } catch (\Exception $e) { return 'y'; } finally { return 'z'; } }
    public function m3() { try { return true; } catch (\Exception $e) { return false; } }
    public function m4() { try { return new A; } catch (\Exception $e) { return new A; } }
    public function m5() { try { return new A; } catch (\Exception $e) { return new B; } }
    public function m6() { try { return 1.5; } catch (\Exception $e) { return 2.5; } }
    public function m7() { try { return [1]; } catch (\Exception $e) { return [2]; } }
    public function m8() { try { return null; } catch (\Exception $e) { return null; } }
    public function m9(?A $a) { try { return $a; } catch (\Exception $e) { return $a; } }
    public function m10() { try { return 1; } catch (\Exception $e) { return 2; } catch (\Error $e) { return 3; } }
    public function m11() { try { return 1; } catch (\Exception $e) { } return 2; }
    public function m12() { try { return 1; } catch (\Exception $e) { throw $e; } }
    public function m13() { try { return 1; } finally { return 2; } }
    public function m14() { try { return 1; } catch (\Exception $e) { return 2; } try { return 'a'; } catch (\Exception $e) { return 'b'; } }
    public function m15(int $i, string $s) { try { return $i; } catch (\Exception $e) { return $s; } }
    public function m16(int $i) { try { return $i; } catch (\Exception $e) { return 5; } }
    public function m17(int $i) { try { return $i + 1; } catch (\Exception $e) { return $i * 2; } }
    public function m18() { try { if ($x) { return 1; } return 2; } catch (\Exception $e) { return 3; } }
    public function m19() { try { return 1; } catch (\Exception $e) { if ($x) { return 'a'; } return 2; } }
    public function m20() { try { $a = 1; return strlen('x'); } catch (\Exception $e) { return 'a' . 'b'; } }
    public function m21(array $a) { try { return $a; } catch (\Exception $e) { return []; } }
    public function m22() { try { return 1; } catch (\Exception $e) { return 2; } finally { echo 1; } }
    public function m23() { try { return 'a'; } catch (\Exception $e) { return 'b'; } finally { return 1; } }
    public function m24(?int $i) { try { return $i; } catch (\Exception $e) { return null; } }
    public function m25() { try { return A::class; } catch (\Exception $e) { return 'b'; } }
}
-----
<?php
namespace App;
class A {} class B extends A {}
final class T
{
    public function m1() { try { return 1; } catch (\Exception $e) { return 'a'; } }
    public function m2(): string { try { return 'x'; } catch (\Exception $e) { return 'y'; } finally { return 'z'; } }
    public function m3(): true { try { return true; } catch (\Exception $e) { return false; } }
    public function m4(): \App\A { try { return new A; } catch (\Exception $e) { return new A; } }
    public function m5() { try { return new A; } catch (\Exception $e) { return new B; } }
    public function m6(): float { try { return 1.5; } catch (\Exception $e) { return 2.5; } }
    public function m7(): array { try { return [1]; } catch (\Exception $e) { return [2]; } }
    public function m8(): null { try { return null; } catch (\Exception $e) { return null; } }
    public function m9(?A $a): ?\App\A { try { return $a; } catch (\Exception $e) { return $a; } }
    public function m10(): int { try { return 1; } catch (\Exception $e) { return 2; } catch (\Error $e) { return 3; } }
    public function m11() { try { return 1; } catch (\Exception $e) { } return 2; }
    public function m12() { try { return 1; } catch (\Exception $e) { throw $e; } }
    public function m13() { try { return 1; } finally { return 2; } }
    public function m14() { try { return 1; } catch (\Exception $e) { return 2; } try { return 'a'; } catch (\Exception $e) { return 'b'; } }
    public function m15(int $i, string $s) { try { return $i; } catch (\Exception $e) { return $s; } }
    public function m16(int $i) { try { return $i; } catch (\Exception $e) { return 5; } }
    public function m17(int $i): int { try { return $i + 1; } catch (\Exception $e) { return $i * 2; } }
    public function m18(): int { try { if ($x) { return 1; } return 2; } catch (\Exception $e) { return 3; } }
    public function m19(): int { try { return 1; } catch (\Exception $e) { if ($x) { return 'a'; } return 2; } }
    public function m20() { try { $a = 1; return strlen('x'); } catch (\Exception $e) { return 'a' . 'b'; } }
    public function m21(array $a) { try { return $a; } catch (\Exception $e) { return []; } }
    public function m22(): int { try { return 1; } catch (\Exception $e) { return 2; } finally { echo 1; } }
    public function m23() { try { return 'a'; } catch (\Exception $e) { return 'b'; } finally { return 1; } }
    public function m24(?int $i) { try { return $i; } catch (\Exception $e) { return null; } }
    public function m25() { try { return A::class; } catch (\Exception $e) { return 'b'; } }
}
