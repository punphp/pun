<?php

declare(strict_types=1);

namespace GuzzleHttp;

final class ProxySelection
{
    private ?string $proxy;

    private bool $bypassed;

    private bool $disabled;

    private function __construct(?string $proxy, bool $bypassed, bool $disabled)
    {}

    public static function none(): self
    {}

    public static function proxy(string $proxy): self
    {}

    public static function bypassed(): self
    {}

    public static function disabled(): self
    {}

    public function getProxy(): ?string
    {}

    public function hasProxy(): bool
    {}

    public function isBypassed(): bool
    {}

    public function isDisabled(): bool
    {}

    public function shouldDisableProxy(): bool
    {}
}
