<?php

namespace App\Probe;

use DateTimeImmutable as Dti;
use stdClass;

interface Shape {}
final class Circle implements Shape {}
enum Suit { case Hearts; }

final class Probe
{
    private ?Circle $circle = null;
    private static ?Circle $shared = null;
    /** @var Circle|null */
    private $doc;

    public function a(?stdClass $a, ?Dti $b, ?Shape $c, ?Suit $d, ?\Closure $e, ?self $f, ?object $g, null|Circle|Shape $h, ?int $i, ?array $j)
    {
        if ($a) { echo 1; }
        if (!$a) { echo 1; } elseif ($b) { echo 2; } elseif (!$c) { echo 3; }
        if ($d) { echo 1; }
        if ($f) { echo 1; }
        if ($g) { echo 1; }
        if ($h) { echo 1; }
        if ($i) { echo 1; }
        if ($j) { echo 1; }
        if (($a)) { echo 1; }
        if (!($a)) { echo 1; }
        if (!!$a) { echo 1; }
        return $a ? 1 : 2;
    }

    public function b(?stdClass $a, ?Dti $b)
    {
        $x = $a ?: 2;
        $y = !$a ? 1 : 2;
        $z = $a === null ? 1 : 2;
        $w = null !== $a ? 1 : 2;
        $v = $a == null;
        $u = $a !== NULL;
        $t = ($a) === (null);
        $s = $a === null || $b === null;
        return [$x, $y, $z, $w, $v, $u, $t, $s];
    }

    public function c(?stdClass $a, ?Dti $b, bool $flag)
    {
        if ($a && $b) { echo 1; }
        if ($a || $b) { echo 1; }
        if (!$a || !$b) { echo 1; }
        if (!$a && !$b) { echo 1; }
        if ($a && $flag) { echo 1; }
        if ($flag && $a && $b) { echo 1; }
        if ($a and $b) { echo 1; }
        if ($a or $b) { echo 1; }
        if (!$a || $b) { echo 1; }
        if ($a xor $b) { echo 1; }
        $r = $a && $b;
        return $r;
    }

    public function d(?stdClass $a, ?Dti $b)
    {
        if (empty($a)) { echo 1; }
        if (!empty($a)) { echo 1; }
        if (empty($this->circle)) { echo 1; }
        if (empty($a->prop)) { echo 1; }
        while ($a) { $a = null; }
        while (!$a) { $a = new stdClass(); }
        for (; $b;) { break; }
    }

    public function d2(?stdClass $a, ?Dti $b, ?Circle $c, ?Dti $b2, ?Dti $b3, ?Circle $c2)
    {
        do { echo 1; } while ($b2);
        do { echo 1; } while (!$b3);
        while ($a === null) { $a = new stdClass(); }
        while (($c2)): echo 1; endwhile;
        if ($c): echo 1; elseif (!$b): echo 2; endif;
        if (!($c === null)) { echo 1; }
        if (!(empty($c))) { echo 1; }
        if ($c ?? $b) { echo 1; }
        if (null === $c || !($b)) { echo 1; }
        $r = empty($c) ? 1 : (!empty($b) ? 2 : 3);
        do { echo 1; } while ($b !== null);
    }

    public function e()
    {
        if ($this->circle) { echo 1; }
        if (!$this->circle) { echo 1; }
        if ($this->circle === null) { echo 1; }
        if (self::$shared) { echo 1; }
        if (self::$shared !== null) { echo 1; }
        if ($this->doc) { echo 1; }
        if ($this->doc === null) { echo 1; }
        if ($this->find()) { echo 1; }
        if ($this->find() === null) { echo 1; }
        if ($this->docFind()) { echo 1; }
        if ($this->docFind() !== null) { echo 1; }
        if ($c = $this->find()) { echo 1; }
        if (null === ($c = $this->find())) { echo 1; }
        if (($c = $this->find()) && $c) { echo 1; }
        while ($c = $this->find()) { echo 1; }
        $local = $this->find();
        if ($local) { echo 1; }
        $fn = fn (?Circle $q) => $q ? 1 : 2;
        return $fn;
    }

    private function find(): ?Circle
    {
        return $this->circle;
    }

    /** @return Circle|null */
    private function docFind()
    {
        return $this->circle;
    }

    /** @param Circle|null $p */
    public function f($p, ?Circle $q)
    {
        if ($p) { echo 1; }
        if ($p === null) { echo 1; }
        if ($q?->x) { echo 1; }
        if ($q /* note */) { echo 1; }
        if ($q
            === null) { echo 1; }
        match (true) { $q === null => 1, default => 2 };
        return $q === null;
    }
}
-----
<?php

namespace App\Probe;

use DateTimeImmutable as Dti;
use stdClass;

interface Shape {}
final class Circle implements Shape {}
enum Suit { case Hearts; }

final class Probe
{
    private ?Circle $circle = null;
    private static ?Circle $shared = null;
    /** @var Circle|null */
    private $doc;

    public function a(?stdClass $a, ?Dti $b, ?Shape $c, ?Suit $d, ?\Closure $e, ?self $f, ?object $g, null|Circle|Shape $h, ?int $i, ?array $j)
    {
        if ($a instanceof \stdClass) { echo 1; }
        if (!$a instanceof \stdClass) { echo 1; } elseif ($b instanceof \DateTimeImmutable) { echo 2; } elseif (!$c instanceof \App\Probe\Shape) { echo 3; }
        if ($d instanceof \App\Probe\Suit) { echo 1; }
        if ($f instanceof \App\Probe\Probe) { echo 1; }
        if ($g) { echo 1; }
        if ($h instanceof \App\Probe\Shape) { echo 1; }
        if ($i) { echo 1; }
        if ($j) { echo 1; }
        if (($a instanceof \stdClass)) { echo 1; }
        if (!$a instanceof \stdClass) { echo 1; }
        if (!!$a) { echo 1; }
        return $a instanceof \stdClass ? 1 : 2;
    }

    public function b(?stdClass $a, ?Dti $b)
    {
        $x = $a ?: 2;
        $y = !$a instanceof \stdClass ? 1 : 2;
        $z = !$a instanceof \stdClass ? 1 : 2;
        $w = $a instanceof \stdClass ? 1 : 2;
        $v = $a == null;
        $u = $a instanceof \stdClass;
        $t = !$a instanceof \stdClass;
        $s = !$a instanceof \stdClass || !$b instanceof \DateTimeImmutable;
        return [$x, $y, $z, $w, $v, $u, $t, $s];
    }

    public function c(?stdClass $a, ?Dti $b, bool $flag)
    {
        if ($a instanceof \stdClass && $b instanceof \DateTimeImmutable) { echo 1; }
        if ($a instanceof \stdClass || $b instanceof \DateTimeImmutable) { echo 1; }
        if (!$a instanceof \stdClass || !$b instanceof \DateTimeImmutable) { echo 1; }
        if (!$a && !$b) { echo 1; }
        if ($a instanceof \stdClass && $flag) { echo 1; }
        if ($flag && $a instanceof \stdClass && $b instanceof \DateTimeImmutable) { echo 1; }
        if ($a and $b) { echo 1; }
        if ($a or $b) { echo 1; }
        if (!$a instanceof \stdClass || $b instanceof \DateTimeImmutable) { echo 1; }
        if ($a xor $b) { echo 1; }
        $r = $a instanceof \stdClass && $b instanceof \DateTimeImmutable;
        return $r;
    }

    public function d(?stdClass $a, ?Dti $b)
    {
        if (!$a instanceof \stdClass) { echo 1; }
        if ($a instanceof \stdClass) { echo 1; }
        if (!$this->circle instanceof \App\Probe\Circle) { echo 1; }
        if (empty($a->prop)) { echo 1; }
        while ($a instanceof \stdClass) { $a = null; }
        while (!$a) { $a = new stdClass(); }
        for (; $b;) { break; }
    }

    public function d2(?stdClass $a, ?Dti $b, ?Circle $c, ?Dti $b2, ?Dti $b3, ?Circle $c2)
    {
        do { echo 1; } while ($b2 instanceof \DateTimeImmutable);
        do { echo 1; } while (!$b3);
        while (!$a instanceof \stdClass) { $a = new stdClass(); }
        while (($c2 instanceof \App\Probe\Circle)): echo 1; endwhile;
        if ($c instanceof \App\Probe\Circle): echo 1; elseif (!$b instanceof \DateTimeImmutable): echo 2; endif;
        if (!(!$c instanceof \App\Probe\Circle)) { echo 1; }
        if ($c instanceof \App\Probe\Circle) { echo 1; }
        if ($c ?? $b) { echo 1; }
        if (!$c instanceof \App\Probe\Circle || !($b instanceof \DateTimeImmutable)) { echo 1; }
        $r = !$c instanceof \App\Probe\Circle ? 1 : ($b instanceof \DateTimeImmutable ? 2 : 3);
        do { echo 1; } while ($b instanceof \DateTimeImmutable);
    }

    public function e()
    {
        if ($this->circle instanceof \App\Probe\Circle) { echo 1; }
        if (!$this->circle instanceof \App\Probe\Circle) { echo 1; }
        if (!$this->circle instanceof \App\Probe\Circle) { echo 1; }
        if (self::$shared instanceof \App\Probe\Circle) { echo 1; }
        if (self::$shared instanceof \App\Probe\Circle) { echo 1; }
        if ($this->doc) { echo 1; }
        if ($this->doc === null) { echo 1; }
        if ($this->find() instanceof \App\Probe\Circle) { echo 1; }
        if (!$this->find() instanceof \App\Probe\Circle) { echo 1; }
        if ($this->docFind()) { echo 1; }
        if ($this->docFind() !== null) { echo 1; }
        if (($c = $this->find()) instanceof \App\Probe\Circle) { echo 1; }
        if (!($c = $this->find()) instanceof \App\Probe\Circle) { echo 1; }
        if (($c = $this->find()) && $c) { echo 1; }
        while ($c = $this->find()) { echo 1; }
        $local = $this->find();
        if ($local instanceof \App\Probe\Circle) { echo 1; }
        $fn = fn (?Circle $q) => $q instanceof \App\Probe\Circle ? 1 : 2;
        return $fn;
    }

    private function find(): ?Circle
    {
        return $this->circle;
    }

    /** @return Circle|null */
    private function docFind()
    {
        return $this->circle;
    }

    /** @param Circle|null $p */
    public function f($p, ?Circle $q)
    {
        if ($p) { echo 1; }
        if ($p === null) { echo 1; }
        if ($q?->x) { echo 1; }
        if ($q instanceof \App\Probe\Circle /* note */) { echo 1; }
        if (!$q instanceof \App\Probe\Circle) { echo 1; }
        match (true) { !$q instanceof \App\Probe\Circle => 1, default => 2 };
        return !$q instanceof \App\Probe\Circle;
    }
}
