<?php
declare(strict_types=1);
class MarkerEdges {
    public int $value;
    public int $other;
    public int $defaulted = 1;
    public array $items;
    public function reference(): bool { $local = 1; $this->value = &$local; return isset($this->value); }
    public function append(): bool { $this->items[] = 1; return isset($this->items); }
    public function compound(): bool { $this->value += 1; return isset($this->value); }
    public function increment(): bool { ++$this->value; return isset($this->value); }
    public function switchContinue(int $i): bool {
        switch ($i) { case 0: $this->value = 1; continue; default: $this->value = 2; }
        return isset($this->value);
    }
    public function switchPartial(int $i): bool {
        switch ($i) { case 0: $this->value = 1; break; }
        return isset($this->value);
    }
    public function switchFallthrough(int $i): bool {
        switch ($i) { case 0: $this->other = 1; default: $this->value = 1; }
        return isset($this->other, $this->value);
    }
    public function switchReturn(int $i): bool {
        switch ($i) { case 0: return false; default: $this->value = 1; }
        return isset($this->value);
    }
    public function doBreak(bool $b): bool {
        do { if ($b) { break; } $this->value = 1; } while (false);
        return isset($this->value);
    }
    public function doAlwaysBreak(): bool {
        do { $this->value = 1; break; } while (true);
        return isset($this->value);
    }
    public function whileBreak(bool $b): bool {
        while (true) { if ($b) { break; } $this->value = 1; break; }
        return isset($this->value);
    }
    public function whileContinue(bool $b): bool {
        while ($b) { if ($b) { $b = false; continue; } $this->value = 1; }
        return isset($this->value);
    }
    public function forOnce(): bool {
        for (;;) { $this->value = 1; break; }
        return isset($this->value);
    }
    public function defaults(): bool { return isset($this->defaulted); }
    public function dynamic(): bool { $name = 'value'; $this->$name = 1; return isset($this->value); }
    public function missing(): bool { $this->missing = 1; return isset($this->missing); }
    public function finallyRead(bool $b): bool {
        try { if ($b) { return false; } $this->value = 1; }
        finally { $read = isset($this->value); }
        return isset($this->value);
    }
    public function finallyCatch(bool $b): bool {
        try { if ($b) { throw new RuntimeException(); } $this->value = 1; }
        catch (RuntimeException $e) { $this->value = 2; }
        finally { $read = isset($this->value); }
        return isset($this->value);
    }
    public function finallyBreak(bool $b): bool {
        do { try { if ($b) { break; } $this->value = 1; }
            finally { $read = isset($this->value); } } while (false);
        return isset($this->value);
    }
    public function finallyWrites(bool $b): bool {
        try { if ($b) { $this->value = 1; } } finally { $this->value = 2; }
        return isset($this->value);
    }
    public function finallyNewBreak(): bool {
        do { try { break; } finally { $this->value = 1; } } while (false);
        return isset($this->value);
    }
    public function finallyNewContinue(array $items): bool {
        foreach ($items as $item) { try { continue; } finally { $this->value = 1; } }
        return isset($this->value);
    }
    public function finallySwitch(int $i): bool {
        switch ($i) { default: try { break; } finally { $this->value = 1; } }
        return isset($this->value);
    }
}
class HookMarkers {
    public int $value;
    public bool $present { get => isset($this->value); }
    public function __construct() { $this->value = 1; }
}
/** @property int $value */
class MagicMarkers {
    public function __set(string $name, mixed $value): void {}
    public function __get(string $name): mixed { return 1; }
    public function test(): bool { $this->value = 1; return isset($this->value); }
}
class FinallyThrowMarkers {
    public int $value;
    public function __construct() { try { $this->value = 1; return; } finally { throw new RuntimeException(); } }
    public function check(): bool { return isset($this->value); }
}
